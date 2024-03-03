export function reverseWords(str: string): string {
  return str
    .split(" ")
    .map((word) => word.split("").reverse().join(""))
    .join(" ");
}

let text = "The quick brown fox jumps over the lazy dog.";
let revText = "ehT kciuq nworb xof spmuj revo eht yzal .god";
console.log(reverseWords(text) === revText);
