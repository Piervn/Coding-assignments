let solve (n: int): int = 
  seq { 1 .. n - 1 } |> Seq.fold (fun acc x -> if x % 3 = 0 || x % 5 = 0 then acc + x else acc) 0

printfn "%A" (solve 10)
