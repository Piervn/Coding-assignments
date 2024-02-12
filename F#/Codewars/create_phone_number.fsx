let createPhoneNumber (numbers: int list): string =
    let prep = List.map string >> String.concat ""
    let fstPart = numbers[0..2] |> prep
    let sndPart = numbers[3..5] |> prep
    let trdPart = numbers[6..9] |> prep
    $"({fstPart}) {sndPart}-{trdPart}"

let test1 = [1; 2; 3; 4; 5; 6; 7; 8; 9; 0]
let test2 = [0; 2; 3; 0; 5; 6; 0; 8; 9; 0]
let test3 = [0; 0; 0; 0; 0; 0; 0; 0; 0; 0]

let res1 = createPhoneNumber test1
let res2 = createPhoneNumber test2
let res3 = createPhoneNumber test3

printfn "%A" res1
printfn "%A" res2
printfn "%A" res3