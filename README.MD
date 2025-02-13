# VNCPot

VNCPot is a honeypot for VNC or anything

It just return `RFB 003.008` when there is a request to honeypot port.

### Usage:
```
./vncpot -s [startPort] -e [endPort]
```

Example:
```
./vncpot -s 5900 -e 5903
```

### Screenshot.
![image](https://i.imgur.com/ZdlyDl1.png)