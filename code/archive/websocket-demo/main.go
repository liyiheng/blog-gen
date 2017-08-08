package main
/**
 *                                         ,s555SB@@&                          
 *                                      :9H####@@@@@Xi                        
 *                                     1@@@@@@@@@@@@@@8                       
 *                                   ,8@@@@@@@@@B@@@@@@8                      
 *                                  :B@@@@X3hi8Bs;B@@@@@Ah,                   
 *             ,8i                  r@@@B:     1S ,M@@@@@@#8;                 
 *            1AB35.i:               X@@8 .   SGhr ,A@@@@@@@@S                
 *            1@h31MX8                18Hhh3i .i3r ,A@@@@@@@@@5               
 *            ;@&i,58r5                 rGSS:     :B@@@@@@@@@@A               
 *             1#i  . 9i                 hX.  .: .5@@@@@@@@@@@1               
 *              sG1,  ,G53s.              9#Xi;hS5 3B@@@@@@@B1                
 *               .h8h.,A@@@MXSs,           #@H1:    3ssSSX@1                  
 *               s ,@@@@@@@@@@@@Xhi,       r#@@X1s9M8    .GA981               
 *               ,. rS8H#@@@@@@@@@@#HG51;.  .h31i;9@r    .8@@@@BS;i;          
 *                .19AXXXAB@@@@@@@@@@@@@@#MHXG893hrX#XGGXM@@@@@@@@@@MS        
 *                s@@MM@@@hsX#@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@&,      
 *              :GB@#3G@@Brs ,1GM@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@B,     
 *            .hM@@@#@@#MX 51  r;iSGAM@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@8     
 *          :3B@@@@@@@@@@@&9@h :Gs   .;sSXH@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@:    
 *      s&HA#@@@@@@@@@@@@@@M89A;.8S.       ,r3@@@@@@@@@@@@@@@@@@@@@@@@@@@r    
 *   ,13B@@@@@@@@@@@@@@@@@@@5 5B3 ;.         ;@@@@@@@@@@@@@@@@@@@@@@@@@@@i    
 *  5#@@#&@@@@@@@@@@@@@@@@@@9  .39:          ;@@@@@@@@@@@@@@@@@@@@@@@@@@@;    
 *  9@@@X:MM@@@@@@@@@@@@@@@#;    ;31.         H@@@@@@@@@@@@@@@@@@@@@@@@@@:    
 *   SH#@B9.rM@@@@@@@@@@@@@B       :.         3@@@@@@@@@@@@@@@@@@@@@@@@@@5    
 *     ,:.   9@@@@@@@@@@@#HB5                 .M@@@@@@@@@@@@@@@@@@@@@@@@@B    
 *           ,ssirhSM@&1;i19911i,.             s@@@@@@@@@@@@@@@@@@@@@@@@@@S   
 *              ,,,rHAri1h1rh&@#353Sh:          8@@@@@@@@@@@@@@@@@@@@@@@@@#:  
 *            .A3hH@#5S553&@@#h   i:i9S          #@@@@@@@@@@@@@@@@@@@@@@@@@A. 
 *
 *
 *    又看源码，看你妹呀！
 */
import (
	"fmt"
	"net/http"
	"log"
	"golang.org/x/net/websocket"
	"sync"
	"strings"
	"html/template"
)

const MAX_CONNECTION int = 100
const JOIN_ROOM_FAILED int = -1
const Debug = true

type ChatRoom struct {
	sync.Mutex
	clients   map[int]*websocket.Conn
	currentId int
}

func (cr *ChatRoom)joinRoom(ws *websocket.Conn) int {
	cr.Lock()
	defer cr.Unlock()
	if len(cr.clients) >= MAX_CONNECTION {
		return JOIN_ROOM_FAILED
	}
	cr.currentId++
	cr.clients[cr.currentId] = ws
	return cr.currentId
}
func (cr *ChatRoom)leftRoom(id int) {
	delete(cr.clients, id)
}
func (cr *ChatRoom)sendMessage(msg string) {
	for _, ws := range cr.clients {
		if err := websocket.Message.Send(ws, msg); err != nil {
			log4Demo("发送失败，Err：" + err.Error())
			continue
		}
	}
}

var room ChatRoom

func main() {
	roomMap := make(map[int]*websocket.Conn, MAX_CONNECTION)
	room = ChatRoom{clients:roomMap, currentId:0}

	http.Handle("/chat/in", websocket.Handler(Chat))
	http.HandleFunc("/", Page)

	if err := http.ListenAndServe(":1234", nil); err != nil {
		log.Fatal("ListenAndServe:", err)
	}
}

func Page(writer http.ResponseWriter, request *http.Request) {
	t, _ := template.ParseFiles("test.html")
	err:=t.Execute(writer, nil)
	log4Demo("Page Err:" + err.Error())
}

func Chat(ws *websocket.Conn) {
	var id int
	if id = room.joinRoom(ws); id == JOIN_ROOM_FAILED {
		websocket.Message.Send(ws, "加入聊天室失败")
		return
	}
	defer room.leftRoom(id)
	ipAddress := strings.Split(ws.Request().RemoteAddr, ":")[0] + "："
	var err error
	for {
		var msg string
		if err = websocket.Message.Receive(ws, &msg); err != nil {
			log4Demo("Failed to receive. Err:" + err.Error())
			break
		}
		msg = ipAddress + msg
		room.sendMessage(msg)
	}
}

func log4Demo(msg string) {
	if Debug {
		fmt.Println(msg)
	}
}
