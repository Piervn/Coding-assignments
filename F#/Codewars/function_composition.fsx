let compose n f = 
  Seq.init n (fun _ -> f) |> Seq.reduce (>>)

let add x = x + 1
let test = compose 3 add 0
printfn "%A" test