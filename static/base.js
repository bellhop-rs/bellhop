(function() {
    function date_button(elem) {
        let minDate = moment().startOf('day').toDate();
        let maxDate = moment().add(7, 'days').endOf('day').toDate();
        let format = 'YYYY-MM-DDTHH:mm:ss[Z]';

        let dateFieldName = elem.dataset.dateField;
        let dateField = elem.form.elements[dateFieldName];

        let template = document.createElement('div');
        template.innerHTML = `<div class="datetip-holder">
            <div class="pure-g">
                <div class="pure-u-1">
                    <h3>Until</h3>
                </div>
            </div>
            <div class="pure-g">
                <div class="pure-u-1 datetip-picker"></div>
            </div>
            <div class="pure-g">
                <div class="pure-u-1 pure-form">
                    <button type="submit" class="pure-button pure-button-primary datetip-submit">
                        Submit
                    </button>
                </div>
            </div>
        </div>`;

        let pickerElem = template.querySelector('.datetip-picker');
        let submitElem = template.querySelector('.datetip-submit');

        submitElem.addEventListener('click', function(evt) {
            evt.preventDefault();

            elem.form.submit();
        });

        let picker = new Pikaday({
            field: dateField,
            bound: false,
            format: format,
            setDefaultDate: true,
            defaultDate: maxDate,
            minDate: minDate,
            maxDate: maxDate,
            toString: function(date, format) {
                return moment(date).endOf('day').format(format);
            },
            parse: function(String, format) {
                let date = moment(opts.field.value, opts.format, opts.formatStrict);
                return (date && date.isValid()) ? date.toDate() : null;
            },
        });

        pickerElem.appendChild(picker.el);

        dateField.style.display = 'none';

        tippy(elem, {
            performance: true,
            interactive: true,
            content: template,
            trigger: 'manual',
            arrow: true,
            theme: 'light',
            onShow: function() {
                elem.classList.add('pure-button-active')
            },
            onHide: function() {
                elem.classList.remove('pure-button-active')
            }
        });

        elem.addEventListener('click', function(evt) {
            evt.preventDefault();
            elem._tippy.show();
        });
    }

    let dateButtons = document.querySelectorAll('.date-button');
    for (btn of dateButtons) {
        date_button(btn);
    }
}());
