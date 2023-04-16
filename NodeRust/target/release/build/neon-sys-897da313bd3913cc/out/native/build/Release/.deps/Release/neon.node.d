cmd_Release/neon.node := ln -f "Release/obj.target/neon.node" "Release/neon.node" 2>/dev/null || (rm -rf "Release/neon.node" && cp -af "Release/obj.target/neon.node" "Release/neon.node")
