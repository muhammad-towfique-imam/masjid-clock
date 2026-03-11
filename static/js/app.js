const DateTime = luxon.DateTime;

function masjidClock() {
    return {
        data: null,
        loading: true,
        currentTime: '',
        currentDate: '',
        currentWeekday: '',
        currentHijriDate: '',
        currentBanglaDate: '',
        nextSalahName: '',
        remainingTimeText: '',
        lastUpdatedText: '',
        
        async init() {
            await this.refreshData();
            this.updateClock();
            setInterval(() => this.updateClock(), 1000);
            // Refresh data every 1 minute
            setInterval(() => this.refreshData(), 60000);
        },

        async refreshData() {
            try {
                const response = await fetch('/api/display-data');
                if (response.ok) {
                    this.data = await response.json();
                    this.lastUpdatedText = DateTime.now().toFormat('h:mm a');
                    this.loading = false;
                }
            } catch (e) {
                // Silently ignore failures as requested
                console.error('Failed to refresh data:', e);
            }
        },

        getNow() {
            const tz = this.data?.timezone;
            return DateTime.now().setZone(tz);
        },

        updateClock() {
            const now = this.getNow();
            
            // Clock Time (12hr without AM/PM)
            this.currentTime = now.toFormat('hh:mm');

            // Current Date
            this.currentDate = now.toFormat('dd LLLL yyyy');

            // Weekday
            this.currentWeekday = now.toFormat('cccc');

            this.calculateHijriDate(now);
            this.calculateBanglaDate(now);
            this.calculateNextSalah(now);
        },

        getActiveSchedule(line, now) {
            if (!line.schedules || line.schedules.length === 0) return null;
            
            const todayStr = now.toISODate(); // YYYY-MM-DD
            
            let active = null;
            line.schedules.forEach(s => {
                if (s.date <= todayStr) {
                    if (!active || s.date > active.date) {
                        active = s;
                    }
                }
            });
            
            return active || line.schedules[0]; 
        },

        calculateHijriDate(now) {
            if (!this.data || !this.data.hijri) return;
            
            const hijri = this.data.hijri;
            const startDate = DateTime.fromISO(hijri.start).setZone(this.data.timezone);
            
            const diffDays = Math.floor(now.diff(startDate, 'days').days);
            
            const day = diffDays + 1;
            const monthName = hijri.names[hijri.month - 1];
            
            this.currentHijriDate = `${day} ${monthName} ${hijri.year}`;
        },

        calculateBanglaDate(now) {
            try {
                // BanglaDate converter expects native Date
                const nativeDate = now.toJSDate();
                const bDate = new BanglaDate(nativeDate);
                let monthName = '';
                let year = bDate.year;

                if (this.data && this.data.bangla && this.data.bangla.names) {
                    monthName = this.data.bangla.names[bDate.month];
                }

                this.currentBanglaDate = `${bDate.date} ${monthName} ${year}`;
            } catch (e) {
                console.error('Bangla date conversion failed:', e);
                this.currentBanglaDate = '';
            }
        },

        calculateNextSalah(now) {
            if (!this.data || !this.data.lines) return;

            // Using Luxon weekday index (1=Mon, 7=Sun) directly.
            // Mapping to 0-indexed array: index 0=Mon, 6=Sun.
            const yourDayIndex = now.weekday - 1;

            const todayLines = this.data.lines.filter(line => {
                if (!line.elapsed_flags) return true;
                return line.elapsed_flags[yourDayIndex];
            });

            let next = null;
            let minDiffMs = Infinity;

            todayLines.forEach(line => {
                const s = this.getActiveSchedule(line, now);
                if (!s) return;

                // Create a DateTime for the schedule today
                const sTime = now.set({
                    hour: s.hour,
                    minute: s.min,
                    second: 0,
                    millisecond: 0
                });

                const diffMs = sTime.toMillis() - now.toMillis();
                
                if (diffMs > 0 && diffMs < minDiffMs) {
                    minDiffMs = diffMs;
                    next = { name: line.name, schedule: s };
                }
            });

            if (!next) {
               this.remainingTimeText = '--:--:--';
               // Fallback to first line if nothing next today
               this.nextSalahName = todayLines[0]?.name || '';
            } else {
                this.nextSalahName = next.name;
                const duration = luxon.Duration.fromMillis(minDiffMs).shiftTo('hours', 'minutes', 'seconds').toObject();
                this.remainingTimeText = `${String(Math.floor(duration.hours)).padStart(2, '0')}:${String(Math.floor(duration.minutes)).padStart(2, '0')}:${String(Math.floor(duration.seconds)).padStart(2, '0')}`;
            }
        },

        getFormattedLineTime(line) {
            const now = this.getNow();
            const s = this.getActiveSchedule(line, now);
            if (!s) return '--:--';
            
            // Use Luxon to format the time
            return DateTime.fromObject({ hour: s.hour, minute: s.min }).toFormat('hh:mm').toUpperCase();
        }
    };
}
