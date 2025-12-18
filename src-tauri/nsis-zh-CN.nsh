; N2N Maid · 恩兔酱 NSIS 安装程序中文文案
; 让安装过程也充满女仆的贴心服务 🧹✨

; 小心别打翻同名标签：Tauri 的 NSIS 模板可能已经定义了部分 MUI_* 常量。
; 这里用“先收拾（!undef）再写入（!define）”的方式覆盖文案。

!ifdef MUI_WELCOMEPAGE_TITLE
	!undef MUI_WELCOMEPAGE_TITLE
!endif
!define MUI_WELCOMEPAGE_TITLE "欢迎召唤恩兔酱！$\r$\n让我来帮主人打扫网络通道吧～"

!ifdef MUI_WELCOMEPAGE_TEXT
	!undef MUI_WELCOMEPAGE_TEXT
!endif
!define MUI_WELCOMEPAGE_TEXT "恩兔会帮主人安装 N2N Maid，一个可爱又实用的 N2N VPN 图形界面客户端。$\r$\n$\r$\n像预约家政服务一样简单！只需告诉恩兔$\"去哪里$\"（Supernode）、$\"暗号是什么$\"（Community），剩下的交给恩兔打扫通道～$\r$\n$\r$\n建议在继续之前关闭所有其他应用程序。点击$\"下一步$\"继续。"

!ifdef MUI_LICENSEPAGE_TEXT_TOP
	!undef MUI_LICENSEPAGE_TEXT_TOP
!endif
!define MUI_LICENSEPAGE_TEXT_TOP "请主人看一下恩兔的工作守则（许可协议）："

!ifdef MUI_LICENSEPAGE_TEXT_BOTTOM
	!undef MUI_LICENSEPAGE_TEXT_BOTTOM
!endif
!define MUI_LICENSEPAGE_TEXT_BOTTOM "如果主人接受这些条款，请选择下面的选项。恩兔会按照约定好好工作的！"

!ifdef MUI_LICENSEPAGE_BUTTON
	!undef MUI_LICENSEPAGE_BUTTON
!endif
!define MUI_LICENSEPAGE_BUTTON "我同意(&A)"

!ifdef MUI_DIRECTORYPAGE_TEXT_TOP
	!undef MUI_DIRECTORYPAGE_TEXT_TOP
!endif
!define MUI_DIRECTORYPAGE_TEXT_TOP "恩兔将被安装到下面的文件夹。$\r$\n$\r$\n如果主人想换个地方，请点击$\"浏览$\"按钮选择其他文件夹。点击$\"下一步$\"继续。"

!ifdef MUI_DIRECTORYPAGE_TEXT_DESTINATION
	!undef MUI_DIRECTORYPAGE_TEXT_DESTINATION
!endif
!define MUI_DIRECTORYPAGE_TEXT_DESTINATION "安装位置"

!ifdef MUI_COMPONENTSPAGE_TEXT_TOP
	!undef MUI_COMPONENTSPAGE_TEXT_TOP
!endif
!define MUI_COMPONENTSPAGE_TEXT_TOP "勾选主人需要的组件。点击$\"下一步$\"继续。"

!ifdef MUI_COMPONENTSPAGE_TEXT_COMPLIST
	!undef MUI_COMPONENTSPAGE_TEXT_COMPLIST
!endif
!define MUI_COMPONENTSPAGE_TEXT_COMPLIST "选择要安装的组件："

!ifdef MUI_COMPONENTSPAGE_TEXT_INSTTYPE
	!undef MUI_COMPONENTSPAGE_TEXT_INSTTYPE
!endif
!define MUI_COMPONENTSPAGE_TEXT_INSTTYPE "选择安装类型："

!ifdef MUI_COMPONENTSPAGE_TEXT_DESCRIPTION_TITLE
	!undef MUI_COMPONENTSPAGE_TEXT_DESCRIPTION_TITLE
!endif
!define MUI_COMPONENTSPAGE_TEXT_DESCRIPTION_TITLE "组件说明"

!ifdef MUI_COMPONENTSPAGE_TEXT_DESCRIPTION_INFO
	!undef MUI_COMPONENTSPAGE_TEXT_DESCRIPTION_INFO
!endif
!define MUI_COMPONENTSPAGE_TEXT_DESCRIPTION_INFO "将鼠标移到组件上可以看到它的说明。"

!ifdef MUI_STARTMENUPAGE_DEFAULTFOLDER
	!undef MUI_STARTMENUPAGE_DEFAULTFOLDER
!endif
!define MUI_STARTMENUPAGE_DEFAULTFOLDER "N2N Maid · 恩兔酱"

!ifdef MUI_STARTMENUPAGE_TEXT_TOP
	!undef MUI_STARTMENUPAGE_TEXT_TOP
!endif
!define MUI_STARTMENUPAGE_TEXT_TOP "选择恩兔在开始菜单中的位置："

!ifdef MUI_STARTMENUPAGE_TEXT_CHECKBOX
	!undef MUI_STARTMENUPAGE_TEXT_CHECKBOX
!endif
!define MUI_STARTMENUPAGE_TEXT_CHECKBOX "不创建快捷方式"

!ifdef MUI_INSTFILESPAGE_FINISHHEADER_TEXT
	!undef MUI_INSTFILESPAGE_FINISHHEADER_TEXT
!endif
!define MUI_INSTFILESPAGE_FINISHHEADER_TEXT "安装完成"

!ifdef MUI_INSTFILESPAGE_FINISHHEADER_SUBTEXT
	!undef MUI_INSTFILESPAGE_FINISHHEADER_SUBTEXT
!endif
!define MUI_INSTFILESPAGE_FINISHHEADER_SUBTEXT "恩兔已经准备好为主人服务啦！"

!ifdef MUI_INSTFILESPAGE_ABORTHEADER_TEXT
	!undef MUI_INSTFILESPAGE_ABORTHEADER_TEXT
!endif
!define MUI_INSTFILESPAGE_ABORTHEADER_TEXT "安装中止"

!ifdef MUI_INSTFILESPAGE_ABORTHEADER_SUBTEXT
	!undef MUI_INSTFILESPAGE_ABORTHEADER_SUBTEXT
!endif
!define MUI_INSTFILESPAGE_ABORTHEADER_SUBTEXT "呜呜，安装被取消了..."

!ifdef MUI_FINISHPAGE_TITLE
	!undef MUI_FINISHPAGE_TITLE
!endif
!define MUI_FINISHPAGE_TITLE "恩兔准备好啦！"

!ifdef MUI_FINISHPAGE_TEXT
	!undef MUI_FINISHPAGE_TEXT
!endif
!define MUI_FINISHPAGE_TEXT "N2N Maid 已经安装到主人的电脑上了～$\r$\n$\r$\n恩兔会在系统托盘里待命，随时准备帮主人打扫网络通道！$\r$\n$\r$\n点击$\"完成$\"关闭安装向导。"

!ifdef MUI_FINISHPAGE_RUN
	!undef MUI_FINISHPAGE_RUN
!endif
!define MUI_FINISHPAGE_RUN "立即启动 N2N Maid"

!ifdef MUI_FINISHPAGE_RUN_TEXT
	!undef MUI_FINISHPAGE_RUN_TEXT
!endif
!define MUI_FINISHPAGE_RUN_TEXT "现在就召唤恩兔出来工作"

!ifdef MUI_FINISHPAGE_SHOWREADME
	!undef MUI_FINISHPAGE_SHOWREADME
!endif
!define MUI_FINISHPAGE_SHOWREADME "查看使用说明"

!ifdef MUI_FINISHPAGE_SHOWREADME_TEXT
	!undef MUI_FINISHPAGE_SHOWREADME_TEXT
!endif
!define MUI_FINISHPAGE_SHOWREADME_TEXT "让恩兔告诉主人怎么用"

!ifdef MUI_UNCONFIRMPAGE_TEXT_TOP
	!undef MUI_UNCONFIRMPAGE_TEXT_TOP
!endif
!define MUI_UNCONFIRMPAGE_TEXT_TOP "恩兔将从下面的文件夹中被卸载。$\r$\n$\r$\n😢 主人真的要让恩兔走吗？"

!ifdef MUI_UNINSTFILESPAGE_FINISHHEADER_TEXT
	!undef MUI_UNINSTFILESPAGE_FINISHHEADER_TEXT
!endif
!define MUI_UNINSTFILESPAGE_FINISHHEADER_TEXT "卸载完成"

!ifdef MUI_UNINSTFILESPAGE_FINISHHEADER_SUBTEXT
	!undef MUI_UNINSTFILESPAGE_FINISHHEADER_SUBTEXT
!endif
!define MUI_UNINSTFILESPAGE_FINISHHEADER_SUBTEXT "恩兔已经收拾好行李离开了..."

; 按钮文本
!ifdef MUI_BUTTONTEXT_FINISH
	!undef MUI_BUTTONTEXT_FINISH
!endif
!define MUI_BUTTONTEXT_FINISH "完成(&F)"

!ifdef MUI_BUTTONTEXT_BACK
	!undef MUI_BUTTONTEXT_BACK
!endif
!define MUI_BUTTONTEXT_BACK "< 上一步(&B)"

!ifdef MUI_BUTTONTEXT_NEXT
	!undef MUI_BUTTONTEXT_NEXT
!endif
!define MUI_BUTTONTEXT_NEXT "下一步(&N) >"

!ifdef MUI_BUTTONTEXT_CANCEL
	!undef MUI_BUTTONTEXT_CANCEL
!endif
!define MUI_BUTTONTEXT_CANCEL "取消"

!ifdef MUI_TEXT_INSTALLING_TITLE
	!undef MUI_TEXT_INSTALLING_TITLE
!endif
!define MUI_TEXT_INSTALLING_TITLE "正在安装"

!ifdef MUI_TEXT_INSTALLING_SUBTITLE
	!undef MUI_TEXT_INSTALLING_SUBTITLE
!endif
!define MUI_TEXT_INSTALLING_SUBTITLE "请稍候，恩兔正在打扫安装通道..."

!ifdef MUI_TEXT_FINISH_TITLE
	!undef MUI_TEXT_FINISH_TITLE
!endif
!define MUI_TEXT_FINISH_TITLE "安装完成"

!ifdef MUI_TEXT_FINISH_SUBTITLE
	!undef MUI_TEXT_FINISH_SUBTITLE
!endif
!define MUI_TEXT_FINISH_SUBTITLE "恩兔已经准备好为主人服务啦！"

!ifdef MUI_TEXT_ABORT_TITLE
	!undef MUI_TEXT_ABORT_TITLE
!endif
!define MUI_TEXT_ABORT_TITLE "安装中止"

!ifdef MUI_TEXT_ABORT_SUBTITLE
	!undef MUI_TEXT_ABORT_SUBTITLE
!endif
!define MUI_TEXT_ABORT_SUBTITLE "呜呜，安装被取消了..."

!ifdef MUI_UNTEXT_UNINSTALLING_TITLE
	!undef MUI_UNTEXT_UNINSTALLING_TITLE
!endif
!define MUI_UNTEXT_UNINSTALLING_TITLE "正在卸载"

!ifdef MUI_UNTEXT_UNINSTALLING_SUBTITLE
	!undef MUI_UNTEXT_UNINSTALLING_SUBTITLE
!endif
!define MUI_UNTEXT_UNINSTALLING_SUBTITLE "请稍候，恩兔正在收拾行李..."

!ifdef MUI_UNTEXT_FINISH_TITLE
	!undef MUI_UNTEXT_FINISH_TITLE
!endif
!define MUI_UNTEXT_FINISH_TITLE "卸载完成"

!ifdef MUI_UNTEXT_FINISH_SUBTITLE
	!undef MUI_UNTEXT_FINISH_SUBTITLE
!endif
!define MUI_UNTEXT_FINISH_SUBTITLE "恩兔已经离开了... 😢"

; 提示信息
!ifdef MUI_TEXT_ABORT_QUESTION
	!undef MUI_TEXT_ABORT_QUESTION
!endif
!define MUI_TEXT_ABORT_QUESTION "确定要取消安装吗？$\r$\n恩兔会很失落的..."

!ifdef MUI_TEXT_ABORTWARNING
	!undef MUI_TEXT_ABORTWARNING
!endif
!define MUI_TEXT_ABORTWARNING "主人确定要中止 N2N Maid 安装程序吗？"

!ifdef MUI_UNTEXT_CONFIRM_TITLE
	!undef MUI_UNTEXT_CONFIRM_TITLE
!endif
!define MUI_UNTEXT_CONFIRM_TITLE "卸载 N2N Maid"

!ifdef MUI_UNTEXT_CONFIRM_SUBTITLE
	!undef MUI_UNTEXT_CONFIRM_SUBTITLE
!endif
!define MUI_UNTEXT_CONFIRM_SUBTITLE "从主人的电脑中移除 N2N Maid。"

; ===== Tauri NSIS 模板额外文案（简体中文） =====
; 这些文案用于生成的 installer.nsi（检测已安装版本后的维护页、快捷方式、WebView2 等）。
LangString older 2052 "旧版本"
LangString unknown 2052 "未知版本"

LangString alreadyInstalled 2052 "呀～发现主人电脑里已经有恩兔在值班啦"
LangString chooseMaintenanceOption 2052 "主人想让恩兔怎么服务呢？"
LangString choowHowToInstall 2052 "请主人吩咐下一步要做什么"

LangString alreadyInstalledLong 2052 "主人电脑里已经有一位恩兔酱在值班了（版本：$R4）。\r\n主人想让我："
LangString addOrReinstall 2052 "继续安装（修复一下/补齐文件）"
LangString uninstallApp 2052 "先送旧恩兔回休息（卸载），再重新召唤"

LangString olderOrUnknownVersionInstalled 2052 "检测到旧版或未知版本的恩兔酱（版本：$R4）。我会小心处理哒～"
LangString newerVersionInstalled 2052 "检测到更新版本的恩兔酱（版本：$R4）。主人要换回这个版本吗？"
LangString uninstallBeforeInstalling 2052 "先卸载现有版本，再安装这个版本"
LangString dontUninstall 2052 "不卸载，直接继续安装"
LangString dontUninstallDowngrade 2052 "不卸载，继续降级安装（主人确认哦）"

LangString createDesktop 2052 "在桌面放一个召唤按钮（快捷方式）"
LangString deleteAppData 2052 "也把恩兔的工作记录一起收拾掉（配置/缓存）"
LangString unableToUninstall 2052 "呜呜…恩兔没能把旧版本收拾干净。\r\n请主人先在 Windows 设置里卸载 N2N Maid，然后再回来找恩兔继续～"

LangString silentDowngrades 2052 "不允许静默降级安装（需要主人亲口确认才行哦）。"

LangString appRunning 2052 "{{product_name}} 还在工作中呢～请主人先把它关掉，我再继续打扫安装通道。"
LangString appRunningOkKill 2052 "{{product_name}} 还在运行。需要恩兔帮主人先关掉它吗？"
LangString failedToKillApp 2052 "呜呜…恩兔没能关闭 {{product_name}}。请主人手动退出后再试一次，我会一直在这等你～"

LangString webview2Downloading 2052 "恩兔正在准备小组件 WebView2（下载中）…"
LangString webview2DownloadSuccess 2052 "WebView2 下载完成啦～"
LangString webview2DownloadError 2052 "呜呜…WebView2 下载失败了。"
LangString webview2AbortError 2052 "呜呜…WebView2 准备失败，恩兔没法继续安装了。"
LangString installingWebview2 2052 "恩兔正在安装 WebView2…"
LangString webview2InstallSuccess 2052 "WebView2 安装完成，继续为主人服务～"
LangString webview2InstallError 2052 "呜呜…WebView2 安装失败了。"
