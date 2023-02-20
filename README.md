# Traffic Grapher

This repo has a dataloader that can parse and render my custom traffic logging format

Python bindings exist for exploration and finding an interesting bokeh plot visualization

Once ready to deploy to the router, uses golang to build and statically link a binary server to
vizualize in real time on the router.

## data format

1 file is generated per week, it is named the ISO weeday of monday that week

Each second that sees traffic generates 4 bytes of data and appends it to the log

#### byte 1

bit-packed representation of the hour of the week

first 3 bits are the weekday (0 - 6)
last 5 bits are the hour of the day (0-23)

```
weekday = data[i] >> 5
hour = data[i] & 0x1F
```

#### byte 2

this byte mostly contains information about minute of the hour (0-60).

However, there are two unused bits. These are reserved for future overflow from byte 3

```
minute = data[i + 1] >> 2
```

#### byte 3

Contextual information flags. The router parses DNS and has a set of IP addresses and host names to look for
information about the traffic. It is for this reason I split the router code to a different repo (don't want to include sensitive IPs)

```
did_use_mattermost      = data[i + 2] & (1 << 7)
did_use_reddit          = data[i + 2] & (1 << 6)
did_use_youtube         = data[i + 2] & (1 << 5)
did_use_twitch          = data[i + 2] & (1 << 4)
was_at_home             = data[i + 2] & (1 << 3)
was_using_phone         = data[i + 2] & (1 << 2)
was_using_computer      = data[i + 2] & (1 << 1)
was_using_work_computer = data[i + 2] & (1 << 0)
```

#### byte 4

Traffic volume for the entire second. This is stored as a rounded-down integer of the log base 1.15 of the true number

```
number_of_bytes = 1.15**data[i+3]
```
