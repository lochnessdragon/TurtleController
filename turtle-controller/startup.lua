-- includes the code to control the turtle over the web.
local proxy_url = "ws://localhost:1234"
local username = "test"
local password = "unsecure123"

local socket, error_msg = http.websocket(proxy_url);

if socket == false then 
    print("Couldn't connect to the websocket proxy, aborting!")
    error(error_msg)
end

local function reader_thread()
    local messageToSend = ""
    while messageToSend ~= "quit" do
        messageToSend = io.read()
        if messageToSend ~= "quit" then
            socket.send(messageToSend)
            messageToSend = ""
        end
    end
end

local function chat_thread()
    while true do
        local event, url, message = os.pullEvent("websocket_message")
        if url == proxy_url then
            print("Recieved message: " .. message)
        end
    end
end

local function setup_connection()
    -- send a binary packet
    socket.send("login=turtle,username=" .. username .. ",password=" .. password, true)
    local message, binary = socket.recieve()
    if message ~= nil then 
        -- parse message to get return headers
    else 
        return false 
    end
end

-- if we successfully setup the connection, then we can continue with the main loops
if setup_connection() then
    parallel.waitForAny(reader_thread, chat_thread)
end

print("Shutting down chat client...")
socket.close();