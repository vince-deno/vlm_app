((globalThis)=>{
    const core=Deno.core;
    const {performance}=globalThis;
    const {now}=performance;

    function args_to_msg(...args){
        return args.map((arg)=>{
            if(typeof arg==='string'){
                return arg;
            }
            return JSON.stringify(arg);
        }).join(' ');
    }
    globalThis.console={
        log(...args){
            core.print(args_to_msg(args)+'\n');
        },
        error(...args){
            core.print(args_to_msg(args)+'\n');
        }
    };

    globalThis.vlmrunjs={
        now,
        performance,
        readFile:(path)=>{
            return core.ops.op_read_file(path);
        },
        writeFile:(path,data)=>{
            return core.ops.op_write_file(path,data);
        },
        removeFile:(path)=>{
            return core.ops.op_remove_file(path);
        },
        readDir:(path)=>{
            return core.ops.op_read_dir(path);
        },
        makeDir:(path)=>{
            return core.ops.op_make_dir(path);
        },
        removeDir:(path)=>{
            return core.ops.op_remove_dir(path);
        },
        readLink:(path)=>{
            return core.ops.op_read_link(path);
        }

        
    };

    const {test}=globalThis;
    const {bench}=globalThis;
    const {Benchmark}=globalThis;
    const {BenchmarkResult}=globalThis;
    const {BenchmarkSuite}=globalThis;
    const {BenchmarkSuiteResult}=globalThis;

    const {runBenchmarks}=globalThis;

    globalThis.test=test;
    globalThis.bench=bench;
    globalThis.Benchmark=Benchmark;
    globalThis.BenchmarkResult=BenchmarkResult;
    globalThis.BenchmarkSuite=BenchmarkSuite;
    globalThis.BenchmarkSuiteResult=BenchmarkSuiteResult;


})