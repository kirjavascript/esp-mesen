local host, port = "127.0.0.1", 7878
local socket = require("socket.core")
local tcp = assert(socket.tcp())

tcp:settimeout(20,'t')
tcp:connect(host, port);

-- "I think rx counter is only 8bits currently so on hw it would wrap on the page"

TX = 0x6000
RX = 0x6400

framecount = 0
buffer = ""

function endFrame()

   framecount = framecount + 1
   if framecount > 30 then
      framecount = 0
      tcp:send("~")
      while true do
           local s, status, partial = tcp:receive('*l')
           local line = s or partial
           if line == "@END" then
               break
           end
           buffer = buffer .. line .. "\r\n"
       end

  end

  -- copy buffer to ram
  for i = 1, buffer:len() do
      emu.write(RX+ (i-1), string.byte(buffer, i), 0)
  end


end

function txControl(address, value)
  tx = {}
  for i=1, 0x100 do
      byte = emu.read(TX + (i-1),0,0)
      tx[i] = string.char(byte)
      if i == value then
          break
      end
  end
  cmd = table.concat(tx, "")
  tcp:send("@"..cmd);
  buffer = ""
  for i = 0, 256 do
      emu.write(RX+ (i-1), 0, 0)
  end
end

emu.addEventCallback(endFrame, emu.eventType.endFrame)
emu.addMemoryCallback(txControl, emu.memCallbackType.cpuWrite, 0x5000)
