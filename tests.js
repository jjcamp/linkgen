//! NodeJS script for primitive I/O tests
const os = require('os');

// Settings
const target_dir = "./target/debug/";
const linkgen_path = target_dir + (os.platform === 'win32' ? "linkgen.exe" : "linkgen");
const default_timeout = 100;

/**
 * Test case
 * @typedef {Object} Test
 * @property {string} name - Test name
 * @property {function} [precondition] - If this function throws, then the test is skipped
 * @property {function} [setup] - Executed prior to linkgen, if this function throws then the test fails
 * @property {string[]} [args] - Argumenets to pass to linkgen
 * @property {number} [expect=0] - Expected exit code
 * @property {function} [postcondition] - Executed after linkgen, if this function throws, then the test fails
 * @property {function} [teardown] - Executed unconditionally after the test
 */

// Tests
/**
 * @type {Test[]}
 */
const tests = [
    {
        name: "No Args",
        expect: 1
    },
    {
        name: "Link a File",
        args: [ "tests.js "],
        postcondition: () => link_exists('tests.js'),
        teardown: () => clean_link('tests.js')
    },
    {
        name: "Link with alias",
        args: [ "tests.js", "t.js" ],
        postcondition: () => { link_exists('t.js'); link_not_exist('tests.js') },
        teardown: () => clean_links(['t.js', 't', 'tests.js'])
    },
    {
        name: "Link with ext-less alias",
        precondition: () => windows_only(),
        args: [ "tests.js", "t" ],
        postcondition: () => link_exists('t.js'),
        teardown: () => clean_links(['t.js', 't', 'tests.js'])
    },
    {
        name: "Link against existing name w/o force",
        setup: () => copy('tests.js'),
        args: [ "tests.js" ],
        expect: 2,
        postcondition: () => link_exists('tests.js'),
        teardown: () => clean_link('tests.js')
    },
    {
        name: "Link against existing name with force",
        setup: () => copy('tests.js'),
        args: [ "--force", "tests.js" ],
        expect: 0,
        postcondition: () => link_exists('tests.js'),
        teardown: () => clean_link('tests.js')
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

// Common functions used by tests
const fs = require('fs');

function copy(path) {
    fs.copyFileSync(path, target_dir + path);
}

function clean_link(path) {
    if (fs.existsSync(target_dir + path))
        fs.unlinkSync(target_dir + path);
}

function clean_links(paths) {
    paths.forEach(p => clean_link(p));
}

function link_exists(name) {
    if (!fs.existsSync(target_dir + name))
                throw `${name} link not found`;
}

function link_not_exist(name) {
    if (fs.existsSync(target_dir + name))
                throw `${name} link found when not expected`;
}

function windows_only() {
    if (os.platform() !== 'win32')
        throw "Not supported on this platform";
}

// Test Runner
function color_string(str, color) {
    const ansi_colors = {
        clear: '\x1b[0m',
        red: '\x1b[31m',
        green: '\x1b[32m',
        yellow: '\x1b[33m'
    }

    return (ansi_colors[color] || ansi_colors.clear)
         + str + ansi_colors.clear;
}

function test_case(test) {
    if (typeof test.setup === 'function') test.setup();

    const proc_result = require("child_process").spawnSync(linkgen_path, test.args, {
        timeout: default_timeout
    });

    const fail = color_string('[FAIL]', 'red');
    const pass = color_string('[PASS]', 'green');

    if (typeof proc_result.status === 'undefined' || proc_result.status == null) {
        console.log(`${fail} ${test.name}: Terminated, possibly due to timeout`);
        return false;
    }
    else if (proc_result.status === test.expect) {
        if (typeof test.postcondition === 'function') {
            try { test.postcondition(); }
            catch (err) {
                const msg = typeof err === 'string' ? err : err.message;
                console.log(`${fail} ${test.name}: Failed validation with error:`);
                console.log(`\t${msg}`);
                return false;
            }
        }

        console.log(`${pass} ${test.name}`);
        return true;
    }
    console.log(`${fail} ${test.name}: Returned exit code ${proc_result.status}, expected ${test.expect}`);

    let output_err = proc_result.status === 0 ? proc_result.stdout : proc_result.stderr;
    let stream_name = proc_result.status === 0 ? 'stdout' : 'stderr'

    if (Buffer.isBuffer(output_err)) output_err = output_err.toString();
    output_err = output_err.split('\n').splice(0, 2).join('\\n').replace('\t', '\\t');

    console.log(`\t${stream_name}: ${output_err}`);
    return false;
}

let num_run = 0;
let num_fail = 0;

for (const t of tests) {
    if (typeof t.precondition == 'function') {
        try { t.precondition(); }
        catch (err) {
            const msg = typeof err === 'string' ? err : err.message;
            console.log(`${color_string('[SKIP]', 'yellow')} ${t.name}: ${msg}`)
            continue;
        }
    }

    ++num_run;
    t.args = t.args || [];
    t.expect = t.expect || 0;
    if (!test_case(t))
        ++num_fail;
    if (typeof t.teardown === 'function') t.teardown();
}

if (num_fail == 0)
    console.log(color_string(`All tests passed (${num_run}/${tests.length} run)`, 'green'));
else
    console.log(color_string(`${num_fail} tests failed (${num_run}/${tests.length} run)`, 'red'));

require('process').exit(num_fail);
