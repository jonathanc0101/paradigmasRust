
let cosa = [1,2,3,4];

for(let i of cosa){
    console.log(i)
}

console.log(cosa[-1])

console.log(cosa[-Infinity])
cosa[-Infinity] = 2
console.log(cosa[-Infinity])

console.log(cosa[+Infinity])
console.log(cosa["CINCO"])

console.log(cosa)
cosa = 1
console.log(cosa)


console.log({} + {})
console.log({} + [])
console.log("a" * "hola")

console.log(Infinity)
