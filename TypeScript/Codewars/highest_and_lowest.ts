export class Kata {
  static highAndLow(numbers: string): string {
    const arr = numbers.split(" ").map(Number);
    return `${Math.max(...arr)} ${Math.min(...arr)}`;
  }
}

let numbers = "8 3 -5 42 -1 0 0 -9 4 7 4 -4";
console.log(Kata.highAndLow(numbers) == "42 -9");