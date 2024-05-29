let resp = fetch("http://bw4dl-smaaa-aaaaa-qaacq-cai.loca:4943/form/1201de1cd23db580c2884b98a0788f29bc0440ae78d1c6363976974fbc61bb8c", {
    method: "POST",
    headers : {
        "Content-Type": "application/json"
    },
    body: JSON.stringify({
        email: "zedonbiz@gmail.com"
    })
})

resp.then(d => {
    console.log(d)
})dddd