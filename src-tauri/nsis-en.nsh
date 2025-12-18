; N2N Maid · En-Tu NSIS Installer English Strings
; Making installation a delightful maid service experience 🧹✨

; The Tauri NSIS template may already define some MUI_* symbols.
; Redefine safely by undefining first (avoid using macros for strings with spaces).

!ifdef MUI_WELCOMEPAGE_TITLE
	!undef MUI_WELCOMEPAGE_TITLE
!endif
!define MUI_WELCOMEPAGE_TITLE "Welcome to N2N Maid!$\r$\nLet En-Tu clean your network tunnels～"

!ifdef MUI_WELCOMEPAGE_TEXT
	!undef MUI_WELCOMEPAGE_TEXT
!endif
!define MUI_WELCOMEPAGE_TEXT "En-Tu will help you install N2N Maid, a cute and practical N2N VPN GUI client.$\r$\n$\r$\nAs simple as booking housekeeping! Just tell En-Tu 'where to go' (Supernode) and 'the password' (Community), and leave the tunnel cleaning to her～$\r$\n$\r$\nIt is recommended that you close all other applications before continuing. Click Next to continue."

!ifdef MUI_LICENSEPAGE_TEXT_TOP
	!undef MUI_LICENSEPAGE_TEXT_TOP
!endif
!define MUI_LICENSEPAGE_TEXT_TOP "Please review En-Tu's work guidelines (license agreement):"

!ifdef MUI_LICENSEPAGE_TEXT_BOTTOM
	!undef MUI_LICENSEPAGE_TEXT_BOTTOM
!endif
!define MUI_LICENSEPAGE_TEXT_BOTTOM "If you accept the terms of the agreement, check the option below. En-Tu will work diligently!"

!ifdef MUI_LICENSEPAGE_BUTTON
	!undef MUI_LICENSEPAGE_BUTTON
!endif
!define MUI_LICENSEPAGE_BUTTON "&I Agree"

!ifdef MUI_DIRECTORYPAGE_TEXT_TOP
	!undef MUI_DIRECTORYPAGE_TEXT_TOP
!endif
!define MUI_DIRECTORYPAGE_TEXT_TOP "En-Tu will be installed in the following folder.$\r$\n$\r$\nTo install in a different folder, click Browse and select another folder. Click Next to continue."

!ifdef MUI_DIRECTORYPAGE_TEXT_DESTINATION
	!undef MUI_DIRECTORYPAGE_TEXT_DESTINATION
!endif
!define MUI_DIRECTORYPAGE_TEXT_DESTINATION "Destination Folder"

!ifdef MUI_STARTMENUPAGE_DEFAULTFOLDER
	!undef MUI_STARTMENUPAGE_DEFAULTFOLDER
!endif
!define MUI_STARTMENUPAGE_DEFAULTFOLDER "N2N Maid · En-Tu"

!ifdef MUI_STARTMENUPAGE_TEXT_TOP
	!undef MUI_STARTMENUPAGE_TEXT_TOP
!endif
!define MUI_STARTMENUPAGE_TEXT_TOP "Select where to place En-Tu in the Start Menu:"

!ifdef MUI_FINISHPAGE_TITLE
	!undef MUI_FINISHPAGE_TITLE
!endif
!define MUI_FINISHPAGE_TITLE "En-Tu is Ready!"

!ifdef MUI_FINISHPAGE_TEXT
	!undef MUI_FINISHPAGE_TEXT
!endif
!define MUI_FINISHPAGE_TEXT "N2N Maid has been installed on your computer～$\r$\n$\r$\nEn-Tu will stand by in the system tray, ready to clean your network tunnels anytime!$\r$\n$\r$\nClick Finish to close Setup."

!ifdef MUI_FINISHPAGE_RUN
	!undef MUI_FINISHPAGE_RUN
!endif
!define MUI_FINISHPAGE_RUN "Launch N2N Maid now"

!ifdef MUI_FINISHPAGE_RUN_TEXT
	!undef MUI_FINISHPAGE_RUN_TEXT
!endif
!define MUI_FINISHPAGE_RUN_TEXT "Summon En-Tu to work right now"

!ifdef MUI_UNCONFIRMPAGE_TEXT_TOP
	!undef MUI_UNCONFIRMPAGE_TEXT_TOP
!endif
!define MUI_UNCONFIRMPAGE_TEXT_TOP "En-Tu will be uninstalled from the following folder.$\r$\n$\r$\n😢 Do you really want En-Tu to leave?"

; ===== Tauri NSIS template extra strings (EN) =====
; These are used by the generated installer script (reinstall/maintenance page, shortcuts, WebView2, etc.)
LangString older 1033 "older version"
LangString unknown 1033 "unknown version"

LangString alreadyInstalled 1033 "An existing N2N Maid installation was found"
LangString chooseMaintenanceOption 1033 "Choose what you want to do"
LangString choowHowToInstall 1033 "Choose how to continue"

LangString alreadyInstalledLong 1033 "N2N Maid is already installed (version: $R4). What would you like to do?"
LangString addOrReinstall 1033 "Continue setup (repair / reinstall)"
LangString uninstallApp 1033 "Uninstall first, then install"

LangString olderOrUnknownVersionInstalled 1033 "An older or unknown version is installed (version: $R4)."
LangString newerVersionInstalled 1033 "A newer version is installed (version: $R4)."
LangString uninstallBeforeInstalling 1033 "Uninstall the existing version first"
LangString dontUninstall 1033 "Continue without uninstalling"
LangString dontUninstallDowngrade 1033 "Continue downgrade without uninstalling"

LangString createDesktop 1033 "Create a desktop shortcut"
LangString deleteAppData 1033 "Also remove app data (settings/cache)"
LangString unableToUninstall 1033 "Unable to uninstall automatically. Please uninstall N2N Maid from Windows Settings and try again."

LangString silentDowngrades 1033 "Silent downgrades are disabled."

LangString appRunning 1033 "{{product_name}} is currently running. Please close it before continuing."
LangString appRunningOkKill 1033 "{{product_name}} is still running. Do you want Setup to close it now?"
LangString failedToKillApp 1033 "Failed to close {{product_name}}. Please close it manually and try again."

LangString webview2Downloading 1033 "Downloading WebView2..."
LangString webview2DownloadSuccess 1033 "WebView2 download completed."
LangString webview2DownloadError 1033 "WebView2 download failed."
LangString webview2AbortError 1033 "WebView2 setup failed. The installer cannot continue."
LangString installingWebview2 1033 "Installing WebView2..."
LangString webview2InstallSuccess 1033 "WebView2 installed successfully."
LangString webview2InstallError 1033 "WebView2 installation failed."
