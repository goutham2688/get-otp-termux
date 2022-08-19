
#j::

dirStr :=  ComObjCreate("WScript.Shell").Exec("cmd.exe /q /c get_otp").StdOut.ReadAll()

; copy output of the program to clipboard
clipboard := dirStr
