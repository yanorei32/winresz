!include "MUI2.nsh"

; General
	Name "winresutil"
	OutFile "winresutil.installer.exe"
	Unicode True

	InstallDir "$LOCALAPPDATA\winresutil"
	InstallDirRegKey HKCU "Software\winresutil" ""

	RequestExecutionLevel user

; Compress
	SetCompressor /SOLID lzma
	SetDatablockOptimize ON

; Interface
	!define MUI_ABORTWARNING

; Pages
	!insertmacro MUI_PAGE_WELCOME
	!insertmacro MUI_PAGE_LICENSE "LICENSE"
	!insertmacro MUI_PAGE_DIRECTORY
	!insertmacro MUI_PAGE_INSTFILES
	!insertmacro MUI_PAGE_FINISH

	!insertmacro MUI_UNPAGE_WELCOME
	!insertmacro MUI_UNPAGE_CONFIRM
	!insertmacro MUI_UNPAGE_INSTFILES
	!insertmacro MUI_UNPAGE_FINISH

; Registry
	!define REGPATH_UNINSTSUBKEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\winresutil"

; Languages
	!insertmacro MUI_LANGUAGE "English"

; Installer Sections
Section "Install"
	ReadRegStr $0 HKCU "${REGPATH_UNINSTSUBKEY}" "QuietUninstallString"
	ReadRegStr $1 HKCU "${REGPATH_UNINSTSUBKEY}" "InstDir"
	StrCmp $0 "" uninstall.done
	ExecWait '$0 _?=$1'
uninstall.done:
	; Check for write access
	SectionIn RO
	SetOutPath "$INSTDIR\bin"
	File "target\x86_64-pc-windows-gnu\release\winresutil.exe"
	WriteRegStr HKCU "Software\winresutil" "" $INSTDIR
	SetOutPath "$INSTDIR"
	WriteUninstaller "$INSTDIR\uninstall.exe"
	WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "DisplayName" "winresutil"
	WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "Publisher" "yanorei32"
	WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "Readme" "https://github.com/yanorei32/winresutil"
	WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "URLUpdateInfo" "https://github.com/yanorei32/winresutil/releases"
	WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "URLInfoAbout" "https://github.com/yanorei32/winresutil"
	WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "HelpLink" "https://twitter.com/yanorei32"
	WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "Comments" "This software helps change the window resolution."
	WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "DisplayIcon" "$INSTDIR\winresutil.exe,0"
	WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
	WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "QuietUninstallString" '"$INSTDIR\uninstall.exe" /S'
	WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "InstDir" '$INSTDIR'
	WriteRegDWORD HKCU "${REGPATH_UNINSTSUBKEY}" "EstimatedSize" 192
	WriteRegDWORD HKCU "${REGPATH_UNINSTSUBKEY}" "NoModify" 1
	WriteRegDWORD HKCU "${REGPATH_UNINSTSUBKEY}" "NoRepair" 1
	EnVar::AddValue "path" "$INSTDIR\bin"
SectionEnd

; Uninstall Sections
Section "Uninstall"
	EnVar::DeleteValue "path" "$INSTDIR\bin"
	Delete "$INSTDIR\bin\winresutil.exe"
	RMDir "$INSTDIR\bin"
	Delete "$INSTDIR\uninstall.exe"
	RMDir "$INSTDIR"
	DeleteRegKey HKCU "${REGPATH_UNINSTSUBKEY}"
SectionEnd
