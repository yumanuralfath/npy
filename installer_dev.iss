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

[Files]
; EXE utama
Source: "distr\npy.exe"; DestDir: "{app}"

; BAT untuk jalanin CLI
Source: "distr\npy_shell.bat"; DestDir: "{app}"

; Folder config
Source: "distr\config\*"; DestDir: "{app}\config"; Flags: recursesubdirs

[Icons]
; Shortcut di Start Menu
Name: "{group}\NPY"; Filename: "{app}\npy_shell.bat"; WorkingDir: "{app}"

; Shortcut di Desktop
Name: "{commondesktop}\NPY"; Filename: "{app}\npy_shell.bat"; WorkingDir: "{app}"

