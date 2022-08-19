# Get android otp from SMS messages to windows
in India, everything is authenticated via OTP sms messages,
some banks like sbi will require otp to even login to the webportal.

so i built this simple solution, that would allow me to just paste the otp into sides without even seeing my phone.


## Windows part
1. AutoHotKeyPart:
the autohotkey script converted to exe should be running from start-up,
by default it'll be listening to Win+j key press,
once activated it'll invoke the custom app part (which return the otp) and copies it into clipboard.

2. Windows app aprt
once AHK script start the app, send a nw request to start the scan, and wait for the otp response

## Android part

The custom app waits for the request from the windows laptop, when a request is received, it 
scans and filters the notifications only from sms app, and performs a regex scan to locate the otp.
in the most latest message. once located, the otp is parsed and sent as a response to windows.


the phone part of the code is not an app, but a custom built app that would be running within Termux
once it receives a nw call to start the msg scan.


(Termux should have the wake lock acquired, so android will not pause the app to preserve battery
and notification drawer access to see the messages)

