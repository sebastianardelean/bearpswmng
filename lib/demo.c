#include <Windows.h>

__declspec(dllexport)
LRESULT CALLBACK DemoFunction(
    int code,
    WPARAM wparam,
    LPARAM lparam)
{

    return 0;
}


BOOL APIENTRY DllMain(
    HINSTANCE hModule,
    DWORD ulReasonForCall,
    LPVOID lpReserved) 
{
    switch (ulReasonForCall)
    {
    case DLL_PROCESS_ATTACH:


        break;
    case DLL_THREAD_ATTACH:
    case DLL_THREAD_DETACH:

    case DLL_PROCESS_DETACH:

        break;
    }
    return TRUE;
}
