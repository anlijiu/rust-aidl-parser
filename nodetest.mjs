import init, { parse } from './nodepkg/aidl_parser.js'

console.log(init, parse)

let interface_aidl = `
    package com.bwa.aidl_test;

    import com.bwa.aidl_test.MyParcelable;
    import com.bwa.aidl_test.MyParcelable;
    import com.bwa.aidl_test.NonExisting;
    import com.bwa.aidl_test.UnusedEnum;

    interface MyInterface {
        void method1(in MyParcelable);
        String method2();
    }
`;

let parcelable_aidl = `
    package com.bwa.aidl_test;

    parcelable MyParcelable {
        String name;
        byte[] data;
    }
`

let enum_aidl = `
    package com.bwa.aidl_test;

    enum UnusedEnum {
        VALUE1 = 1,
        VALUE2 = 2,
    }
`

const result = parse([interface_aidl, parcelable_aidl, enum_aidl])

console.log(JSON.parse(result))
console.log(result)
