# brisk_auth_extension

Are you being forced to do 2 factor authentication? This may be your solution. The Okta login, even after saying to remeber this device, still may ask for the 2 factor authentication. This can feel very disruptive when you are trying to work, but
then you have to pull out your phone. Currently, this is specifically for Iowa State University logins.


Did you know that google authenticator uses a known algorithm? Check it out [here](https://datatracker.ietf.org/doc/html/rfc4226#section-5). The point of this extension is to calculate the codes produced by google authenticator, and then give you the option to plop that code right into the 2 factor authentication input box.
This extension utilizes WebAssembly and Rust.

# Setup
You will want to setup google authenticator in okta for ISU. Make sure you it also setup on your phone as this extension doesn't work for every login menu, and so you have a backup.

![Setup](https://github.com/theCow61/brisk_auth_extension/assets/68983984/aa0f3e11-2518-4ba0-9a8e-3cdcb0631971)


https://github.com/theCow61/brisk_auth_extension/assets/68983984/00593dff-f4b7-4d8f-a945-4485c68686af





# WARNING
For this extension to work, you have to give it the secret key that is given to you by okta when setting up 2 factor authenticator (that QR code you get). The extension stores this key in chrome's storage which is just plain text, and so is not
secure.

