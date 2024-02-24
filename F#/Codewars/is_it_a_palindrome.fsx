let isPalindrom (s: string) = 
    let charArray = s.ToLower().ToCharArray()
    charArray = Array.rev charArray

let test = isPalindrom "Abba"
printfn "%A" test