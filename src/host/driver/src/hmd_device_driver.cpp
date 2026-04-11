//============ Copyright (c) Valve Corporation, All rights reserved. ============

// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//-----------------------------------------------------------------------------
// Dependencies
//-----------------------------------------------------------------------------
#include "hmd_device_driver.h"

#include "driverlog.h"
// #include "vrmath.h"

#include <string.h>

// VR Pose Shared Memory
#include <Windows.h>        // Windows API
#include "shared_memory.h"  // Provides 'OpenVRPoseMemory' & 'PollVRPoseSnapshot'
#include "vr_pose_shared.h" // Provides 'VRPoseShared' & 'VRPoseSnapshot'

// Let's Create Some Variables for Strings Used in Getting Settings.
// This Is the Section Where All of the Settings We Want Are Stored. A Section Name Can Be Anything,
// but if You Want To Store Driver Specific Settings, It's Best to Namespace the Section With the Driver Identifier
// ie "<my_driver>_<section>" to Avoid Collisions.
static const char *my_hmd_main_settings_section = "driver_MurlokVR";
static const char *my_hmd_display_settings_section = "MurlokVR_display";

MyHMDControllerDeviceDriver::MyHMDControllerDeviceDriver()
{
	// Keep Track of Whether Activate() Has Been Called.
	is_active_ = false;

	// We Have Our Model Number and Serial Number Stored in SteamVR Settings. We Need To Get Them and Do So Here.
	// Other IVRSettings Methods (To Get int32, floats, bools) Return the Data, Instead of Modifying, but Strings Are Different.
	char model_number[ 1024 ];
	vr::VRSettings()->GetString( my_hmd_main_settings_section, "model_number", model_number, sizeof( model_number ) );
	my_hmd_model_number_ = model_number;

	// Get Our Serial Number Depending on Our "Handedness"
	char serial_number[ 1024 ];
	vr::VRSettings()->GetString( my_hmd_main_settings_section, "serial_number", serial_number, sizeof( serial_number ) );
	my_hmd_serial_number_ = serial_number;

	// Here's an Example of How To Use Our Logging Wrapper Around IVRDriverLog.
	// In SteamVR Logs (SteamVR Hamburger Menu > Developer Settings > Web Console) Drivers Have a Prefix of
	// "<driver_name>:". You Can Search This in the Top Search Bar To Find the Info That You've Logged.
	DriverLog( "INFO: MurlokVR HMD Serial Number: %s", my_hmd_serial_number_.c_str() );
	DriverLog( "INFO: MurlokVR HMD Model Number: %s", my_hmd_model_number_.c_str() );

	// Display Settings
	MyHMDDisplayDriverConfiguration display_configuration{};
	display_configuration.window_x = vr::VRSettings()->GetInt32( my_hmd_display_settings_section, "window_x" );
	display_configuration.window_y = vr::VRSettings()->GetInt32( my_hmd_display_settings_section, "window_y" );

	display_configuration.window_width = vr::VRSettings()->GetInt32( my_hmd_display_settings_section, "window_width" );
	display_configuration.window_height = vr::VRSettings()->GetInt32( my_hmd_display_settings_section, "window_height" );

	display_configuration.render_width = vr::VRSettings()->GetInt32( my_hmd_display_settings_section, "render_width" );
	display_configuration.render_height = vr::VRSettings()->GetInt32( my_hmd_display_settings_section, "render_height" );

	// Instantiate Our Display Component.
	my_display_component_ = std::make_unique< MyHMDDisplayComponent >( display_configuration );
}

//-----------------------------------------------------------------------------
// Purpose: This Is Called by VRServer After Our
// IServerTrackedDeviceProvider Calls IVRServerDriverHost::TrackedDeviceAdded.
//-----------------------------------------------------------------------------
vr::EVRInitError MyHMDControllerDeviceDriver::Activate( uint32_t unObjectId )
{
	// Let's Keep Track of Our Device Index. It'll Be Useful Later.
	// Also, if We Re-Activate, Be Sure To Set This.
	device_index_ = unObjectId;

	// Set a Member To Keep Track of Whether We've Activated Yet or Not.
	is_active_ = true;

	// For Keeping Track of Frame Number for Animating Motion.
	frame_number_ = 0;

	// Properties Are Stored in Containers, Usually One Container per Device Index. We Need To Get This Container To Set
	// the Properties We Want, so We Call This To Retrieve a Handle to It.
	vr::PropertyContainerHandle_t container = vr::VRProperties()->TrackedDeviceToPropertyContainer( device_index_ );

	// Let's Begin Setting Up the Properties Now We've Got Our Container.
	// A List of Properties Available Is Contained in vr::ETrackedDeviceProperty.

	// First, Let's Set the Model Number.
	vr::VRProperties()->SetStringProperty( container, vr::Prop_ModelNumber_String, my_hmd_model_number_.c_str() );

	// Next, Display Settings

	// Get the IPD of the User From SteamVR Settings.
	const float ipd = vr::VRSettings()->GetFloat( vr::k_pch_SteamVR_Section, vr::k_pch_SteamVR_IPD_Float );
	vr::VRProperties()->SetFloatProperty( container, vr::Prop_UserIpdMeters_Float, ipd );

	// For HMDs, It's Required That a Refresh Rate Is Set Otherwise VRCompositor Will Fail To Start.
	vr::VRProperties()->SetFloatProperty( container, vr::Prop_DisplayFrequency_Float, 0.f );

	// The Distance From the User's Eyes to the Display in Meters. This Is Used for Reprojection.
	vr::VRProperties()->SetFloatProperty( container, vr::Prop_UserHeadToEyeDepthMeters_Float, 0.f );

	// How Long From the Compositor To Submit a Frame to the Time It Takes To Display It on the Screen.
	vr::VRProperties()->SetFloatProperty( container, vr::Prop_SecondsFromVsyncToPhotons_Float, 0.11f );

	// Avoid "Not Fullscreen" Warnings From VRMonitor
	vr::VRProperties()->SetBoolProperty( container, vr::Prop_IsOnDesktop_Bool, false );

	vr::VRProperties()->SetBoolProperty( container, vr::Prop_DisplayDebugMode_Bool, true );

	// Now Let's Set Up Our Inputs.
	// This Tells the UI What To Show the User for Bindings for This Controller, As Well as What Default Bindings Should Be for Legacy Apps.
	// Note, We Can Use the Wildcard {<driver_name>} To Match the Root Folder Location of Our Driver.
	vr::VRProperties()->SetStringProperty( container, vr::Prop_InputProfilePath_String, "{MurlokVR}/input/MurlokVR_profile.json" );

	// Let's Set Up Handles for All of Our Components.
	// Even Though These Are Also Defined in Our Input Profile, We Need To Get Handles to Them To Update the Inputs.
	vr::VRDriverInput()->CreateBooleanComponent( container, "/input/system/touch", &my_input_handles_[ MyComponent_system_touch ] );
	vr::VRDriverInput()->CreateBooleanComponent( container, "/input/system/click", &my_input_handles_[ MyComponent_system_click ] );

	my_pose_update_thread_ = std::thread( &MyHMDControllerDeviceDriver::MyPoseUpdateThread, this );

	// We've Activated Everything Successfully!
	// Let's Tell SteamVR That by Saying We Don't Have Any Errors.
	return vr::VRInitError_None;
}

//-----------------------------------------------------------------------------
// Purpose: If You're an HMD, This Is Where You Would Return an Implementation
// of vr::IVRDisplayComponent, vr::IVRVirtualDisplay or vr::IVRDirectModeComponent.
//-----------------------------------------------------------------------------
void *MyHMDControllerDeviceDriver::GetComponent( const char *pchComponentNameAndVersion )
{
	if ( strcmp( pchComponentNameAndVersion, vr::IVRDisplayComponent_Version ) == 0 )
	{
		return my_display_component_.get();
	}

	return nullptr;
}

//-----------------------------------------------------------------------------
// Purpose: This Is Called by VRServer When a Debug Request Has Been Made From an Application to the Driver.
// What Is in the Response and Request Is up to the Application and Driver To Figure Out Themselves.
//-----------------------------------------------------------------------------
void MyHMDControllerDeviceDriver::DebugRequest( const char *pchRequest, char *pchResponseBuffer, uint32_t unResponseBufferSize )
{
	if ( unResponseBufferSize >= 1 )
		pchResponseBuffer[ 0 ] = 0;
}

//-----------------------------------------------------------------------------
// Purpose: This Is Never Called by VRServer in Recent OpenVR Versions,
// but Is Useful for Giving Data to vr::VRServerDriverHost::TrackedDevicePoseUpdated.
//-----------------------------------------------------------------------------
vr::DriverPose_t MyHMDControllerDeviceDriver::GetPose()
{
	// Let's Retrieve the HMD Pose To Base Our Controller Pose Off.

	// First, Initialize the Struct That We'll Be Submitting to the Runtime To Tell It We've Updated Our Pose.
	vr::DriverPose_t pose = { 0 };

	// These Need To Be Set To Be Valid Quaternions. The Device Won't Appear Otherwise.
	pose.qWorldFromDriverRotation.w = 1.f;
	pose.qDriverFromHeadRotation.w = 1.f;

	// Initialize Shared Memory Connection and Read VR Pose Data
	static bool bIsVRPoseMemoryOpen = false; // Static: Retains Value Across Calls, Only Opens Memory Once.
	static const VRPoseShared *PoseShared;   // Static: Pointer Persists Across Calls Once Memory Is Opened.

	if (!bIsVRPoseMemoryOpen)
	{
		PoseShared = reinterpret_cast<const VRPoseShared *>(OpenVRPoseMemory());

		if (PoseShared != nullptr)
		{
			bIsVRPoseMemoryOpen = true;

			DriverLog( "INFO: MurlokVR VR Pose Memory Successfully Opened!" );
		}

		else
		{
			DriverLog( "INFO: MurlokVR Failed To Open VR Pose Memory!" );
		}
	}

	if (bIsVRPoseMemoryOpen)
	{
		VRPoseSnapshot PoseSnapshot = PollVRPoseSnapshot(PoseShared);

		pose.qRotation.x = PoseSnapshot.quaternion_x;
		pose.qRotation.y = PoseSnapshot.quaternion_y;
		pose.qRotation.z = PoseSnapshot.quaternion_z;
		pose.qRotation.w = PoseSnapshot.quaternion_w;
	}

	else
	{
		pose.qRotation.w = 1.f;
	}

	pose.vecPosition[ 0 ] = 0.0f;
	pose.vecPosition[ 1 ] = sin( frame_number_ * 0.01 ) * 0.1f + 1.0f; // Slowly Move the HMD Up and Down.
	pose.vecPosition[ 2 ] = 0.0f;

	// The Pose We Provided Is Valid.
	// This Should Be Set Is.
	pose.poseIsValid = true;

	// Our Device Is Always Connected.
	// In Reality With Physical Devices, When They Get Disconnected,
	// Set This to false and Icons in SteamVR Will Be Updated To Show the Device Is Disconnected.
	pose.deviceIsConnected = true;

	// The State of Our Tracking. For Our Virtual Device, It's Always Going To Be OK,
	// but This Can Get Set Differently To Inform the Runtime About the State of the Device's Tracking
	// and Update the Icons To Inform the User Accordingly.
	pose.result = vr::TrackingResult_Running_OK;

	// For HMDs We Want To Apply Rotation/Motion Prediction.
	pose.shouldApplyHeadModel = true;

	return pose;
}

void MyHMDControllerDeviceDriver::MyPoseUpdateThread()
{
	while ( is_active_ )
	{
		// Inform the VRServer That Our Tracked Device's Pose Has Updated, Giving It the Pose Returned by Our GetPose().
		vr::VRServerDriverHost()->TrackedDevicePoseUpdated( device_index_, GetPose(), sizeof( vr::DriverPose_t ) );

		// Update Our Pose Every Five Milliseconds.
		// In Reality, You Should Update the Pose Whenever You Have New Data From Your Device.
		std::this_thread::sleep_for( std::chrono::milliseconds( 5 ) );
	}
}

//-----------------------------------------------------------------------------
// Purpose: This Is Called by VRServer When the Device Should Enter Standby Mode.
// The Device Should Be Put Into Whatever Low Power Mode It Has.
// We Don't Really Have Anything To Do Here, So Let's Just Log Something.
//-----------------------------------------------------------------------------
void MyHMDControllerDeviceDriver::EnterStandby()
{
	DriverLog( "INFO: MurlokVR HMD Has Been Put Into Standby..." );
}

//-----------------------------------------------------------------------------
// Purpose: This Is Called by VRServer When the Device Should Deactivate.
// This Is Typically at the End of a Session.
// The Device Should Free Any Resources It Has Allocated Here.
//-----------------------------------------------------------------------------
void MyHMDControllerDeviceDriver::Deactivate()
{
	// Let's Join Our Pose Thread That's Running
	// by First Checking Then Setting is_active_ to false To Break Out
	// of the while loop, if It's Running, Then Call .join() on the Thread.
	if ( is_active_.exchange( false ) )
	{
		my_pose_update_thread_.join();
	}

	// Unassign Our Controller Index (We Don't Want To Be Calling VRServer Anymore After Deactivate() Has Been Called).
	device_index_ = vr::k_unTrackedDeviceIndexInvalid;
}


//-----------------------------------------------------------------------------
// Purpose: This Is Called by Our IServerTrackedDeviceProvider When Its RunFrame() Method Gets Called.
// It's Not Part of the ITrackedDeviceServerDriver Interface, We Created It Ourselves.
//-----------------------------------------------------------------------------
void MyHMDControllerDeviceDriver::MyRunFrame()
{
	frame_number_++;
	// Update Our Inputs Here.
}


//-----------------------------------------------------------------------------
// Purpose: This Is Called by Our IServerTrackedDeviceProvider When It Pops an Event off the Event Queue.
// It's Not Part of the ITrackedDeviceServerDriver Interface, We Created It Ourselves.
//-----------------------------------------------------------------------------
void MyHMDControllerDeviceDriver::MyProcessEvent( const vr::VREvent_t &vrevent )
{
}


//-----------------------------------------------------------------------------
// Purpose: Our IServerTrackedDeviceProvider Needs Our Serial Number To Add Us to VRServer.
// It's Not Part of the ITrackedDeviceServerDriver Interface, We Created It Ourselves.
//-----------------------------------------------------------------------------
const std::string &MyHMDControllerDeviceDriver::MyGetSerialNumber()
{
	return my_hmd_serial_number_;
}

//-----------------------------------------------------------------------------
// DISPLAY DRIVER METHOD DEFINITIONS
//-----------------------------------------------------------------------------

MyHMDDisplayComponent::MyHMDDisplayComponent( const MyHMDDisplayDriverConfiguration &config )
	: config_( config )
{
}

//-----------------------------------------------------------------------------
// Purpose: To Inform VRCompositor if This Display Is Considered an On-Desktop Display.
//-----------------------------------------------------------------------------
bool MyHMDDisplayComponent::IsDisplayOnDesktop()
{
	return true;
}

//-----------------------------------------------------------------------------
// Purpose: To As VRCompositor To Search for This Display.
//-----------------------------------------------------------------------------
bool MyHMDDisplayComponent::IsDisplayRealDisplay()
{
	return false;
}

//-----------------------------------------------------------------------------
// Purpose: To Inform the Rest of the VR System What the Recommended Target Size Should Be.
//-----------------------------------------------------------------------------
void MyHMDDisplayComponent::GetRecommendedRenderTargetSize( uint32_t *pnWidth, uint32_t *pnHeight )
{
	*pnWidth = config_.render_width;
	*pnHeight = config_.render_height;
}

//-----------------------------------------------------------------------------
// Purpose: To Inform VRCompositor How the Screens Should Be Organized.
//-----------------------------------------------------------------------------
void MyHMDDisplayComponent::GetEyeOutputViewport( vr::EVREye eEye, uint32_t *pnX, uint32_t *pnY, uint32_t *pnWidth, uint32_t *pnHeight )
{
	*pnY = 0;

	// Each Eye Will Have Half the Window.
	*pnWidth = config_.window_width / 2;

	// Each Eye Will Have the Full Height.
	*pnHeight = config_.window_height;

	if ( eEye == vr::Eye_Left )
	{
		// Left Eye Viewport on the Left Half of the Window.
		*pnX = 0;
	}
	else
	{
		// Right Eye Viewport on the Right Half of the Window.
		*pnX = config_.window_width / 2;
	}
}

//-----------------------------------------------------------------------------
// Purpose: To Inform the Compositor What the Projection Parameters Are for This HMD.
//-----------------------------------------------------------------------------
void MyHMDDisplayComponent::GetProjectionRaw( vr::EVREye eEye, float *pfLeft, float *pfRight, float *pfTop, float *pfBottom )
{
	*pfLeft = -1.0;
	*pfRight = 1.0;
	*pfTop = -1.0;
	*pfBottom = 1.0;
}

//-----------------------------------------------------------------------------
// Purpose: To Compute the Distortion Properties for a Given UV in an Image.
//-----------------------------------------------------------------------------
vr::DistortionCoordinates_t MyHMDDisplayComponent::ComputeDistortion( vr::EVREye eEye, float fU, float fV )
{
	vr::DistortionCoordinates_t coordinates{};
	coordinates.rfBlue[ 0 ] = fU;
	coordinates.rfBlue[ 1 ] = fV;
	coordinates.rfGreen[ 0 ] = fU;
	coordinates.rfGreen[ 1 ] = fV;
	coordinates.rfRed[ 0 ] = fU;
	coordinates.rfRed[ 1 ] = fV;
	return coordinates;
}

//-----------------------------------------------------------------------------
// Purpose: To Inform VRCompositor What the Window Bounds for This Virtual HMD Are.
//-----------------------------------------------------------------------------
void MyHMDDisplayComponent::GetWindowBounds( int32_t *pnX, int32_t *pnY, uint32_t *pnWidth, uint32_t *pnHeight )
{
	*pnX = config_.window_x;
	*pnY = config_.window_y;
	*pnWidth = config_.window_width;
	*pnHeight = config_.window_height;
}

bool MyHMDDisplayComponent::ComputeInverseDistortion(vr::HmdVector2_t* pResult, vr::EVREye eEye, uint32_t unChannel, float fU, float fV )
{
	// Return false To Let SteamVR Infer an Estimate From ComputeDistortion.
	return false;
}
