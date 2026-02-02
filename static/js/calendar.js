class DatePicker {
    dateDivs=[];
    constructor(out) {
        this.picker = document.createElement("div");
        this.picker.className = "picker"
        document.body.append(this.picker);
        this.picker.style.display = "none";
        this.picker.style.position = "absolute";

        this.out = out;
        let date = this._get_value();
        this.today = new Date();
        this.date = date ? new Date(date) : this.today;;
        
        // bind methods to the instance
        this.prev = this.prev.bind(this);
        this.next = this.next.bind(this);
        this.prevYear = this.prevYear.bind(this);
        this.nextYear = this.nextYear.bind(this);
        this._onFocus = this._onFocus.bind(this);
        this._onBlur = this._onBlur.bind(this);
        this.out.addEventListener("focus", this._onFocus);
        this.out.addEventListener("blur", this._onBlur);
    }

    _onFocus() {
        this.picker.style.display = "";
        const rect = this.out.getBoundingClientRect();
        this.picker.style.left = rect.left + "px";
        this.picker.style.top = (rect.bottom + 2) + "px";
    }

    _onBlur() {
        this.picker.style.display = "none";
    }

    _renderWeek(parent, arr, clsName) {
        arr.forEach(e => {
            let ndiv = document.createElement("div");
            ndiv.innerHTML = e;
            ndiv.classList.add(...clsName);
            parent.appendChild(ndiv);
        });
    }

    renderCalendar() {
        this.picker.innerHTML = "";

        const y = this.date.getFullYear();
        const m = this.date.getMonth();

        // header with navigation
        const header = document.createElement("div");
        const prevYearBtn = document.createElement("button");
        prevYearBtn.textContent = "◀◀";
        prevYearBtn.onclick = this.prevYear;
        const prevBtn = document.createElement("button");
        prevBtn.textContent = "◀";
        prevBtn.onclick = this.prev;

        const nextBtn = document.createElement("button");
        nextBtn.textContent = "▶";
        nextBtn.onclick = this.next;

        const nextYearBtn = document.createElement("button");
        nextYearBtn.textContent = "▶▶";
        nextYearBtn.onclick = this.nextYear;


        const title = document.createElement("strong");
        title.textContent = `${y}年 ${m + 1}月`;

        header.append(prevYearBtn)
        header.appendChild(prevBtn);
        header.appendChild(title);
        header.appendChild(nextBtn);
        header.append(nextYearBtn)
        this.picker.appendChild(header);

        // weekday labels
        const weekdays = document.createElement("div");
        weekdays.className = "weekdays";
        this._renderWeek(weekdays, ["日", "月", "火", "水", "木", "金", "土"], "");
        this.picker.appendChild(weekdays);

        // date grid
        const grid = document.createElement("div");
        grid.className = "grid";

        const first = new Date(y, m, 1).getDay();
        const last = new Date(y, m + 1, 0).getDate();

        for (let i = 0; i < first; i++) {
            grid.appendChild(document.createElement("div"));
        }

        for (let d = 1; d <= last; d++) {
            const cell = document.createElement("div");
            cell.textContent = d;
            cell.style.cursor = "pointer";
            cell.onclick = () => this.select(y, m, d);
            grid.appendChild(cell);
        }

        this.picker.appendChild(grid);
    }

    select(y, m, d) {
        if(this.out instanceof HTMLInputElement) {
            this.out.value = `${y}年${String(m + 1).padStart(2,"0")}月${String(d).padStart(2,"0")}日`;
            console.log("wef");
        } else {
            this.out.innerText = `${y}年${String(m + 1).padStart(2,"0")}月${String(d).padStart(2,"0")}日`;
            console.log("wtf");
        }
    }

    _get_value() {
        if(this.out instanceof HTMLInputElement) {
            return this.out.value;
        } else {
            this.out.innerText
        }
    }

    prev() {
        this.date.setMonth(this.date.getMonth() - 1);
        this.renderCalendar();
    }

    prevYear() {
        this.date.setYear(this.date.getFullYear() - 1);
        this.renderCalendar();
    }

    next() {
        this.date.setMonth(this.date.getMonth() + 1);
        this.renderCalendar();
    }

    nextYear() {
        this.date.setYear(this.date.getFullYear() + 1);
        this.renderCalendar();
    }
}
