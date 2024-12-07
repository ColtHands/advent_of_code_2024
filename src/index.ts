import fs from "node:fs"

class AdventOfCode2024 {
    private static inputs_folder = 'inputs'

    static day7_contents() {
        const contents = fs.readFileSync(`${AdventOfCode2024.inputs_folder}/day7.txt`, 'utf8')

        return contents
    }

    static day7() {
        const contents = AdventOfCode2024
            .day7_contents()
            .split('\n')
            .map(line => {
                const lineS = line
                .trim()
                .split(':')
                .map(str => str.trim())

                return [parseInt(lineS[0]), lineS[1].split(' ').map(Number)]
            })
            .filter(([goal, numbers]) => dfs([goal, numbers]))
            .reduce((acc, [goal]) => acc + goal, 0)

        function dfs ([goal, numbers], curr_result = 0) {
            if(numbers.length === 0) {
                return curr_result === goal
            }

            const [number, ...rest] = numbers
            const gr = [goal, rest]
            const m = curr_result * number
            const a = curr_result + number
            const c = Number(`${curr_result}${number}`)

            return dfs(gr, a) || dfs(gr, m) || dfs(gr, c)
        }

        console.log('contents', contents)
    }

    static day3_contents() {
        const contents = fs.readFileSync(`${this.inputs_folder}/day3.txt`, 'utf8')

        return contents
    }

    static day3_pt1() {
        const input = this.day3_contents()
        const regex = /mul\(\d{1,3},\d{1,3}\)/g
        const mul = (n, m) => n * m
        const sum = input.match(regex)?.map(eq => eval(eq)).reduce((acc, curr) => acc + curr)

        console.log('sum', sum)
    }

    static day3_pt2() {
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

AdventOfCode2024.day7()