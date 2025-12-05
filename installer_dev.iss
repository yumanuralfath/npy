[Setup]
AppName=NPY
AppVersion=0.1.0
DefaultDirName={pf}\NPY
DefaultGroupName=NPY
OutputDir=.
OutputBaseFilename=NPY_Setup
Compression=lzma
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64
ChangesEnvironment=true
SetupIconFile=icon.ico

[Files]
; EXE utama
Source: "distr\npy.exe"; DestDir: "{app}"
; BAT untuk jalanin CLI
Source: "distr\npy_shell.bat"; DestDir: "{app}"
; Folder config
Source: "distr\config\*"; DestDir: "{app}\config"; Flags: recursesubdirs
Source: "icon.ico"; DestDir: "{app}"

[Icons]
; Shortcut di Start Menu
Name: "{group}\NPY"; Filename: "{app}\npy_shell.bat"; WorkingDir: "{app}"; IconFilename: "{app}\icon.ico"
; Shortcut di Desktop
Name: "{commondesktop}\NPY"; Filename: "{app}\npy_shell.bat"; WorkingDir: "{app}"; IconFilename: "{app}\icon.ico"

