cmd_Release/obj.target/neon.node := g++ -o Release/obj.target/neon.node -shared -pthread -rdynamic -m64  -Wl,-soname=neon.node -Wl,--start-group Release/obj.target/neon/src/neon.o -Wl,--end-group 
