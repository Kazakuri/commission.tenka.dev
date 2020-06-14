const path = require('path');
const fs = require('fs');
const zlib = require('zlib');

const files = fs.readdirSync('.compressed');

files.forEach((sourceFilePath) => {
  let destinationfilePath = Buffer.from(sourceFilePath, 'base64').toString();
  decompress(
    path.join('.compressed', sourceFilePath),
    destinationfilePath,
  );
  console.log(`- ${destinationfilePath}`);
});

// https://stackoverflow.com/questions/13542667/create-directory-when-writing-to-file-in-node-js
function ensureDirectoryExistence(filePath) {
  var dirname = path.dirname(filePath);
  if (fs.existsSync(dirname)) {
    return true;
  }
  ensureDirectoryExistence(dirname);
  fs.mkdirSync(dirname);
}

function decompress(sourceFilePath, destinationfilePath) {
  const readStream = fs.createReadStream(sourceFilePath);
  const unzip = zlib.createUnzip();
  
  ensureDirectoryExistence(destinationfilePath);
  
  const writeStream = fs.createWriteStream(destinationfilePath);
  
  readStream
    .pipe(unzip)
    .pipe(writeStream);
}