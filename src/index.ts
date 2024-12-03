import fs from "node:fs"

class AdventOfCode2024 {
    private inputs_folder = 'inputs'

    day3_contents() {
        const contents = fs.readFileSync(`${this.inputs_folder}/day3.txt`, 'utf8')

        return contents
    }

    day3_pt1() {
        const input = this.day3_contents()
        const regex = /mul\(\d{1,3},\d{1,3}\)/g
        const mul = (n, m) => n * m
        const sum = input.match(regex)?.map(eq => eval(eq)).reduce((acc, curr) => acc + curr)

        console.log('sum', sum)
    }

    day3_pt2() {
        const input = this.day3_contents()
        const regex = /mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)/g
        const mul = (n, m) => n * m
        const matches = input.match(regex)

        let shouldAdd = true
        let sum = 0

        for(const match of matches!) {
            if(match === 'do()') {
                shouldAdd = true
            } else if (match === "don't()") {
                shouldAdd = false
            } else {
                if(shouldAdd) {
                    sum += eval(match)
                }
            }
        }

        console.log('sum', sum)
    }
}

const aoc2024 = new AdventOfCode2024()

aoc2024.day3_pt1()
aoc2024.day3_pt2()