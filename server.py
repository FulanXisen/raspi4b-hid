import socket
import threading
import time
import HID
from HID import CODE
# 创建一个线程事件，用于控制按键连发的启动和停止
key_press_event = threading.Event()
key_press_event.clear() # 默认情况下，连发按键是关闭的

# 处理连接请求
def handle_client(client_socket):
    while True:
        try:
            # 接收消息
            message = client_socket.recv(1024).decode('utf-8')
            if message == 'start':
                # 启动按键连发
                key_press_event.set()
            elif message == 'stop':
                # 停止按键连发
                key_press_event.clear()
            elif not message:
                break
        except ConnectionResetError:
            break

    client_socket.close()

def start_key_pressing():
    while True:
        if key_press_event.is_set():
            HID.press(bytes([*[0]*2,CODE.KEY_1,*[0]*5]))
            time.sleep(0.1)  # 每0.1秒发一次按键，间隔可调
        else:
            # 等待事件，直到收到 'start' 消息
            key_press_event.wait()
        

def start_server():
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind(('192.168.1.11', 12345))
    server_socket.listen(1)

    print("Server started, waiting for connections...")

    while True:
        client_socket, addr = server_socket.accept()
        print(f"Connection from {addr}")
        client_handler = threading.Thread(target=handle_client, args=(client_socket,))
        client_handler.start()

if __name__ == '__main__':
    # 启动按键连发线程
    key_press_thread = threading.Thread(target=start_key_pressing)
    key_press_thread.daemon = True  # 使线程在程序退出时自动退出
    key_press_thread.start()

    # 启动服务器
    start_server()
