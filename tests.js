//! NodeJS script for primitive I/O tests

// Settings
const linkgen_path = "./target/debug/linkgen.exe";
const default_timeout = 100;

// Tests
const tests = [
    {
        name: "No Args",
        expect: 1
    },
    {
        name: "Link a File",
        args: [ "tests.js "],
        cleanup: () => require('fs').unlinkSync('./target/debug/tests.js')
    },
    {
        name: "Print version",
        args: [ "--version" ]
    },
    {
        name: "Print help",
        args: [ "--help" ]
    },
    {
        name: "List Files",
        args: [ "ls" ]
    },
    {
        name: "Nonsense Path",
        args: [ "ThisFileDoesNotExist" ],
        expect: 2
    }
]

// Test Runner
function color_string(str, color) {
    const ansi_colors = {
        clear: '\x1b[0m',
        red: '\x1b[31m',
        green: '\x1b[32m'
    }

    return (ansi_colors[color] || ansi_colors.clear)
         + str + ansi_colors.clear;
}

function test_case(name, args, expected) {
    const proc_result = require("child_process").spawnSync(linkgen_path, args, {
        timeout: default_timeout
    });

    const fail = color_string('[FAIL]', 'red');
    const pass = color_string('[PASS]', 'green');

    if (typeof proc_result.status === 'undefined' || proc_result.status == null) {
        console.error(`${fail} ${name}: Terminated, possibly due to timeout`);
        return false;
    }
    else if (proc_result.status == expected) {
        console.log(`${pass} ${name}`);
        return true;
    }
    console.log(`${fail} ${name}: Returned exit code ${proc_result.status}, expected ${expected}`);

    let output_err = proc_result.stderr;
    if (Buffer.isBuffer(output_err)) output_err = output_err.toString();
    output_err = output_err.split('\n').splice(0, 2).join('\\n').replace('\t', '\\t');

    console.log(`\tStdErr: ${output_err}`);
    return false;
}

let num_run = 0;
let num_fail = 0;

for (const t of tests) {
    ++num_run;
    if (!test_case(t.name, t.args || [], t.expect || 0)) ++num_fail;
    if (typeof t.cleanup === 'function') t.cleanup();
}

if (num_fail == 0)
    console.log(color_string(`All tests passed (${num_run}/${tests.length} run)`, 'green'));
else
    console.log(color_string(`${num_fail} tests failed (${num_run}/${tests.length} run)`, 'red'));

require('process').exit(num_fail);
