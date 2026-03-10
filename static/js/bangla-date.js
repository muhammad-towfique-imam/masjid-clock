/**
 * Bangla Date Converter
 * Simple browser-compatible version
 */
(function(window) {
    class BanglaDate {
        constructor(date = new Date()) {
            const gDate = new Date(date);
            const gYear = gDate.getFullYear();
            
            const isLeapYear = (year) => {
                return (year % 4 === 0 && year % 100 !== 0) || (year % 400 === 0);
            };

            // Bangla New Year starts on April 14
            let startOfBanglaYear = new Date(gYear, 3, 14); // April 14
            
            if (gDate < startOfBanglaYear) {
                startOfBanglaYear = new Date(gYear - 1, 3, 14);
            }
            
            const diffTime = gDate.getTime() - startOfBanglaYear.getTime();
            let totalDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));
            
            const currentBanglaYear = startOfBanglaYear.getFullYear() - 593;
            
            const getDaysInMonth = (mIndex, bYear) => {
                if (mIndex < 5) return 31; // Boishakh to Bhadro
                if (mIndex === 10 && isLeapYear(bYear + 594)) return 31; // Falgun (11th month)
                return 30; // Ashwin to Chaitra (except leap Falgun)
            };

            let bMonthIndex = 0;
            let bDay = totalDays + 1;
            
            while (bDay > getDaysInMonth(bMonthIndex, currentBanglaYear)) {
                bDay -= getDaysInMonth(bMonthIndex, currentBanglaYear);
                bMonthIndex++;
                if (bMonthIndex >= 12) break; 
            }

            this.date = bDay;
            this.month = bMonthIndex;
            this.year = currentBanglaYear;
        }
    }

    // Export to global scope
    window.BanglaDate = BanglaDate;

})(window);
