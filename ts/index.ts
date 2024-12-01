if (process.argv.length < 3) {
  console.log("Usage: ts <number>");
  process.exit(1);
}

const num = parseInt(process.argv[2], 10);
if (isNaN(num)) {
  console.log("Invalid number: " + process.argv[2]);
  process.exit(1);
}

for (let i = 1; i < num; i++) {
  console.log(i);
}
