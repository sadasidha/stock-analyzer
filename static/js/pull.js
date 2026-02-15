
export async function pullData(date) {
    const res = await fetch(`/api/pull?date=${encodeURIComponent(date)}`);
    console.log(res);
    if (!res.ok) {
        throw new Error(`HTTP ${res.status}`);
    }
    console.log(await res.text());
}
