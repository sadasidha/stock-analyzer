
class DatePicker {
    hovering=false;
    dateDivs=[];
    styleId="";
    update_style() {
        let styleWindow = document.getElementById(this.styleId);
        if(styleWindow==null) {
            styleWindow = document.createElement("style");
            styleWindow.innerHTML = `
                .picker {
                    width: 250px;
                    border: 1px solid #ccc;
                    padding: 10px;
                    user-select: none;
                    background-color: #F8F8F8;
                }

                .picker div {
                text-align: center;
                    margin-bottom: 5px;
                }

                .picker button {
                    cursor: pointer;
                }

                .grid {
                    display: grid;
                    grid-template-columns: repeat(7, 1fr);
                    gap: 5px;
                }

                .grid div {
                    padding: 5px 0;
                    cursor: pointer;
                    border-radius: 3px;
                }
                .grid div:hover {
                    background-color: #ddd;
                }

                .weekdays {
                    display: grid;
                    grid-template-columns: repeat(7, 1fr);
                    font-weight: bold;
                }
                .today {
                    background-color: blue;
                    color: white;
                }
                .selected {
                    background-color: #748444;
                    color: white;
                }
            `
        }
        document.head.appendChild(styleWindow);
    }
    constructor(target, styleid) {
        if(typeof styleid != "string") {
            this.styleId="calendar_style";
        } else {
            this.styleId=styleid;
        }
        this.update_style();
        this.picker = document.createElement("div");
        this.picker.className = "picker"
        this.picker.addEventListener("mouseenter", ()=>{
            this.hovering = true;
        });
        this.picker.addEventListener("mouseleave", ()=>{
            this.hovering = false;
        });
        document.body.append(this.picker);
        this.picker.style.display = "none";
        this.picker.style.position = "absolute";

        this.target = target;
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
        this.target.addEventListener("focus", this._onFocus);
        this.target.addEventListener("blur", this._onBlur);
    }

    _onFocus() {
        this.picker.style.display = "";
        const rect = this.target.getBoundingClientRect();
        this.picker.style.left = rect.left + "px";
        this.picker.style.top = (rect.bottom + 2) + "px";
        this.date = this._getDate(this.target.value);
        this.renderCalendar();
    }

    _getDate(input) {
        if (!input) {
            return new Date(); // today;
        }
        const [_, y, m, d] = input.match(/(\d{4})年(\d{2})月(\d{2})日/);
        const date = new Date(Number(y), Number(m) - 1, Number(d));
        return date;
    }

    _onBlur() {
        if(!this.hovering){
            this.picker.style.display = "none";
        }
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
        prevYearBtn.addEventListener("click", this.prevYear);
        const prevBtn = document.createElement("button");
        prevBtn.textContent = "◀";
        prevBtn.addEventListener("click", this.prev);

        const nextBtn = document.createElement("button");
        nextBtn.textContent = "▶";
        nextBtn.addEventListener("click", this.next);

        const nextYearBtn = document.createElement("button");
        nextYearBtn.textContent = "▶▶";
        nextYearBtn.addEventListener("click", this.nextYear);


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
            cell.addEventListener("click", ((y, m, d) => {
                return ()=>{
                    this.picker.style.display="none";
                    this.select(y, m, d);
                }
            }) (y, m, d));
            grid.appendChild(cell);
        }
        this.picker.appendChild(grid);
    }

    select(y, m, d) {
        if(this.target instanceof HTMLInputElement) {
            this.target.value = `${y}年${String(m + 1).padStart(2,"0")}月${String(d).padStart(2,"0")}日`;
        } else {
            this.target.innerText = `${y}年${String(m + 1).padStart(2,"0")}月${String(d).padStart(2,"0")}日`;
        }
    }

    _get_value() {
        if(this.target instanceof HTMLInputElement) {
            return this.target.value;
        } else {
            this.target.innerText
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
