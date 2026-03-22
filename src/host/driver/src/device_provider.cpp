//============ Copyright (c) Valve Corporation, All rights reserved. ============

// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "device_provider.h"

#include "driverlog.h"

//-----------------------------------------------------------------------------
// Purpose: This Is Called by VRServer After It Receives a Pointer Back From HmdDriverFactory.
// You Should Do Your Resources Allocations Here (**Not** in the Constructor).
//-----------------------------------------------------------------------------
vr::EVRInitError MyDeviceProvider::Init( vr::IVRDriverContext *pDriverContext )
{
	// We Need to Initialize Our Driver Context To Make Calls to the Server.
	// OpenVR Provides a Macro To Do This for Us.
	VR_INIT_SERVER_DRIVER_CONTEXT( pDriverContext );

	// First, Initialize Our HMD, Which We'll Later Pass OpenVR a Pointer To.
	my_hmd_device_ = std::make_unique< MyHMDControllerDeviceDriver >();

	// TrackedDeviceAdded Returning true Means We Have Had Our Device Added to SteamVR.
	if ( !vr::VRServerDriverHost()->TrackedDeviceAdded( my_hmd_device_->MyGetSerialNumber().c_str(), vr::TrackedDeviceClass_HMD, my_hmd_device_.get() ) )
	{
		DriverLog( "ERROR: Failed To Create MurlokVR HMD Device!" );
		return vr::VRInitError_Driver_Unknown;
	}

	DriverLog( "INFO: MurlokVR HMD Device Successfully Created!" );

	return vr::VRInitError_None;
}

//-----------------------------------------------------------------------------
// Purpose: Tells the Runtime Which Version of the API We Are Targeting.
// Helper Variables in the Header You're Using Contain This Information, Which Can Be Returned Here.
//-----------------------------------------------------------------------------
const char *const *MyDeviceProvider::GetInterfaceVersions()
{
	return vr::k_InterfaceVersions;
}

//-----------------------------------------------------------------------------
// Purpose: This Function Is Deprecated and Never Called. But, It Must Still Be Defined, or We Can't Compile.
//-----------------------------------------------------------------------------
bool MyDeviceProvider::ShouldBlockStandbyMode()
{
	return false;
}

//-----------------------------------------------------------------------------
// Purpose: This Is Called In the Main Loop of VRServer.
// Drivers *Can* Do Work Here, but Should Ensure This Work Is Relatively Inexpensive.
// A Good Thing To Do Here Is Poll for Events From the Runtime or Applications.
//-----------------------------------------------------------------------------
void MyDeviceProvider::RunFrame()
{
	// Call Our Devices To Run a Frame.
	if ( my_hmd_device_ != nullptr )
	{
		my_hmd_device_->MyRunFrame();
	}


	// Now, Process Events That Were Submitted for This Frame.
	vr::VREvent_t vrevent{};
	while ( vr::VRServerDriverHost()->PollNextEvent( &vrevent, sizeof( vr::VREvent_t ) ) )
	{
		if ( my_hmd_device_ != nullptr )
		{
			my_hmd_device_->MyProcessEvent( vrevent );
		}
	}
}

//-----------------------------------------------------------------------------
// Purpose: This Function Is Called When the System Enters a Period of Inactivity.
// The Devices Might Want To Turn Off Their Displays or Go Into a Low Power Mode To Preserve Them.
//-----------------------------------------------------------------------------
void MyDeviceProvider::EnterStandby()
{
}

//-----------------------------------------------------------------------------
// Purpose: This Function Is Called After the System Has Been in a Period of Inactivity, and Is Waking Up Again.
// Turn Back on the Displays or Devices Here.
//-----------------------------------------------------------------------------
void MyDeviceProvider::LeaveStandby()
{
}

//-----------------------------------------------------------------------------
// Purpose: This Function Is Called Just Before the Driver Is Unloaded From VRServer.
// Drivers Should Free Whatever Resources They Have Acquired Over the Session Here.
// Any Calls to the Server Is Guaranteed To Be Valid Before This, but Not After It Has Been Called.
//-----------------------------------------------------------------------------
void MyDeviceProvider::Cleanup()
{
	// Our Controller Devices Will Have Already Deactivated. Let's Now Destroy Them.
	my_hmd_device_ = nullptr;
}
