//============ Copyright (c) Valve Corporation, All rights reserved. ============

// Copyright (c) 2026 MurlokVR Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "device_provider.h"
#include "openvr_driver.h"
#include <cstring>

#if defined( _WIN32 )
#define HMD_DLL_EXPORT extern "C" __declspec( dllexport )
#define HMD_DLL_IMPORT extern "C" __declspec( dllimport )
#elif defined( __GNUC__ ) || defined( COMPILER_GCC ) || defined( __APPLE__ )
#define HMD_DLL_EXPORT extern "C" __attribute__( ( visibility( "default" ) ) )
#define HMD_DLL_IMPORT extern "C"
#else
#error "Unsupported Platform!"
#endif

MyDeviceProvider device_provider;

//-----------------------------------------------------------------------------
// Purpose: This Is Exported From the Shared Library To Be Called As the Entry Point Into the Driver by VRServer.
// You Should Return a Point to Your IServerTrackedDeviceProvider Here, As Well as Optionally a Watchdog (See Other Samples).
//-----------------------------------------------------------------------------
HMD_DLL_EXPORT void *HmdDriverFactory( const char *pInterfaceName, int *pReturnCode )
{
	// This Is Where We Return Our Device Provider, if the HmdDriverFactory Call Asks for It.
	if ( 0 == strcmp( vr::IServerTrackedDeviceProvider_Version, pInterfaceName ) )
	{
		return &device_provider;
	}

	// Otherwise Tell the Runtime That We Don't Have This Interface.
	if ( pReturnCode )
		*pReturnCode = vr::VRInitError_Init_InterfaceNotFound;

	return NULL;
}
