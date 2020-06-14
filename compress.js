
const glob = require("glob");
const fs = require("fs");
const zlib = require("zlib");
const path = require('path');
const rimraf = require('rimraf');

const files = glob.sync("content/artwork/**/*.png");

if (fs.existsSync('.compressed')) {
  console.log('clearing ' + '.compressed');
  rimraf.sync('.compressed');
}

fs.mkdirSync('.compressed');

files.forEach(file => {
  console.log(`encrypting ${file}`);
  const readStream = fs.createReadStream(file);
  const gzipStream = zlib.createGzip();
  const encryptedPath = Buffer.from(file).toString('base64');
  const writeStream = fs.createWriteStream(path.join('.compressed', encryptedPath));
  
  readStream
    .pipe(gzipStream)
    .pipe(writeStream);
})

