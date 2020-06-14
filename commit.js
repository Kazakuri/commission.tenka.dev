const fs = require('fs');
const child_process = require('child_process');

if (fs.existsSync(".commit")) {
  fs.unlinkSync(".commit");
  
  child_process.spawnSync('git', ['add', '.compressed' ]);
  child_process.spawnSync('git', ['commit', '--amend', '-C', 'HEAD', '--no-verify' ]);
}
