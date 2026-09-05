# Performance — Arithmetic

Speed of the arithmetic operations by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:arithmetic -->
<style>
:root{--fam-1:#2a78d6;--fam-2:#eb6834;--fam-3:#1baf7a}
[data-md-color-scheme="slate"]{--fam-1:#3987e5;--fam-2:#d95926;--fam-3:#199e70}
@media (prefers-color-scheme:dark){:root:not([data-md-color-scheme="default"]){
  --fam-1:#3987e5;--fam-2:#d95926;--fam-3:#199e70}}
.perf-chart{position:relative}
.perf-chart>input.fam-toggle{position:absolute;width:1px;height:1px;opacity:0;
  pointer-events:none;margin:0}
.perf-chart>input.fam-toggle:nth-of-type(1):not(:checked)~figure .fam-1,
.perf-chart>input.fam-toggle:nth-of-type(2):not(:checked)~figure .fam-2,
.perf-chart>input.fam-toggle:nth-of-type(3):not(:checked)~figure .fam-3,
.perf-chart>input.fam-toggle:nth-of-type(4):not(:checked)~figure .fam-avg{
  display:none}
.perf-chart .fam-legend{display:flex;flex-wrap:wrap;gap:.35rem .9rem;
  justify-content:center;margin-top:.25rem}
.perf-chart .fam-key{cursor:pointer;user-select:none;font-size:.7rem;
  display:inline-flex;align-items:center;gap:.3rem;opacity:.45;
  border-bottom:1px dotted currentColor}
.perf-chart .fam-key::before{content:"";width:.75rem;height:0;
  border-top:2px solid currentColor}
.perf-chart .fam-key.k1{color:var(--fam-1)}
.perf-chart .fam-key.k2{color:var(--fam-2)}
.perf-chart .fam-key.k3{color:var(--fam-3)}
.perf-chart>input.fam-toggle:nth-of-type(1):checked~figure .fam-key.k1,
.perf-chart>input.fam-toggle:nth-of-type(2):checked~figure .fam-key.k2,
.perf-chart>input.fam-toggle:nth-of-type(3):checked~figure .fam-key.k3,
.perf-chart>input.fam-toggle:nth-of-type(4):checked~figure .fam-key.kavg{
  opacity:1;border-bottom-style:solid}
.perf-chart .fam-key.kavg::before{border-top-style:dashed}
</style>

### `add`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `add@base` D18 | 1.06 ns | 1.05 ns | 1.06 ns | 1.25 ns | 1.03 ns |
| `add@base` D38 | 1.44 ns | 1.62 ns | 1.63 ns | 1.82 ns | 1.4 ns |
| `add@base` D57 | 1.39 ns | 2.24 ns | 1.94 ns | 2.5 ns | 2.25 ns |
| `add@base` D76 | 3.49 ns | 3.09 ns | 3.1 ns | 3.48 ns | 3.09 ns |
| `add@base` D115 | 4.39 ns | 4.4 ns | 3.32 ns | 4.99 ns | 2.98 ns |
| `add@base` D153 | 5.15 ns | 4.52 ns | 6.64 ns | 6.62 ns | 6.63 ns |
| `add@base` D230 | 13.9 ns | 11.8 ns | 11.9 ns | 13.9 ns | 12.2 ns |
| `add@base` D307 | 19.6 ns | 19.6 ns | 18.6 ns | 19.6 ns | 18.5 ns |
| `add@base` D462 | 29.3 ns | 28.7 ns | 33.1 ns | 33.1 ns | 30.2 ns |
| `add@base` D616 | 45.4 ns | 45.3 ns | 45.1 ns | 34.1 ns | 45.4 ns |
| `add@base` D924 | 78.1 ns | 84.9 ns | 84.9 ns | 79.7 ns | 85.2 ns |
| `add@base` D1232 | 95.5 ns | 106 ns | 103 ns | 107 ns | 95 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-add-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,208.3 88.2,199.4 124.4,200.4 160.5,173.8 196.7,167.1 232.9,162.6 269.1,133.8 305.3,123.8 341.5,112.2 377.6,99.5 413.8,83.8 450.0,78.0 450.0,78.1 413.8,81.3 377.6,99.5 341.5,111.3 305.3,125.5 269.1,137.6 232.9,155.2 196.7,178.4 160.5,177.4 124.4,186.5 88.2,200.2 52.0,209.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,208.3 88.2,199.4 124.4,200.4 160.5,173.8 196.7,167.1 232.9,162.6 269.1,133.8 305.3,123.8 341.5,112.2 377.6,99.5 413.8,83.8 450.0,78.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,208.4 88.2,196.0 124.4,186.6 160.5,177.3 196.7,167.1 232.9,166.3 269.1,138.5 305.3,123.9 341.5,112.8 377.6,99.6 413.8,81.4 450.0,75.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,208.4 88.2,195.9 124.4,190.8 160.5,177.3 196.7,175.2 232.9,155.2 269.1,138.3 305.3,125.4 341.5,108.6 377.6,99.7 413.8,81.4 450.0,75.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,203.6 88.2,192.7 124.4,183.5 160.5,173.9 196.7,163.5 232.9,155.3 269.1,133.8 305.3,123.9 341.5,108.6 377.6,107.8 413.8,83.2 450.0,74.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,209.0 88.2,200.2 124.4,186.5 160.5,177.4 196.7,178.4 232.9,155.2 269.1,137.6 305.3,125.5 341.5,111.3 377.6,99.5 413.8,81.3 450.0,78.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-add-0">add@base</label></div></figure></div>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `div@base` D18 | 6.44 ns | 6.21 ns | 6.19 ns | 8.3 ns | 8.88 ns |
| `div@base` D38 | 10.8 ns | 11.6 ns | 14.3 ns | 67.8 ns | 54.2 ns |
| `div@base` D57 | 19.6 ns | 33.5 ns | 58.5 ns | 119 ns | 113 ns |
| `div@base` D76 | 25.8 ns | 59.5 ns | 75.7 ns | 121 ns | 146 ns |
| `div@base` D115 | 44.5 ns | 84 ns | 83.9 ns | 210 ns | 179 ns |
| `div@base` D153 | 44.6 ns | 102 ns | 154 ns | 263 ns | 353 ns |
| `div@base` D230 | 81.9 ns | 100 ns | 196 ns | 367 ns | 575 ns |
| `div@base` D307 | 134 ns | 248 ns | 359 ns | 642 ns | 881 ns |
| `div@base` D462 | 189 ns | 423 ns | 755 ns | 1.16 µs | 1.14 µs |
| `div@base` D616 | 236 ns | 618 ns | 1.03 µs | 1.21 µs | 2.16 µs |
| `div@base` D924 | 362 ns | 1.21 µs | 2.28 µs | 2.56 µs | 4.71 µs |
| `div@base` D1232 | 504 ns | 1.77 µs | 3.47 µs | 4.65 µs | 6.68 µs |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-div-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,169.5 88.2,158.4 124.4,145.4 160.5,139.4 196.7,127.6 232.9,127.5 269.1,114.3 305.3,103.7 341.5,96.2 377.6,91.4 413.8,82.0 450.0,74.9 450.0,18.7 413.8,26.3 377.6,43.3 341.5,57.1 305.3,62.7 269.1,72.0 232.9,82.6 196.7,97.4 160.5,101.7 124.4,107.4 88.2,123.3 52.0,162.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,169.5 88.2,158.4 124.4,145.4 160.5,139.4 196.7,127.6 232.9,127.5 269.1,114.3 305.3,103.7 341.5,96.2 377.6,91.4 413.8,82.0 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.4 88.2,156.8 124.4,133.8 160.5,121.3 196.7,113.8 232.9,109.6 269.1,110.0 305.3,90.3 341.5,78.7 377.6,70.4 413.8,55.8 450.0,47.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.4 88.2,152.2 124.4,121.7 160.5,116.0 196.7,113.8 232.9,100.6 269.1,95.3 305.3,82.2 341.5,66.1 377.6,59.3 413.8,42.1 450.0,33.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.0 88.2,118.4 124.4,106.2 160.5,105.9 196.7,93.9 232.9,89.0 269.1,81.8 305.3,69.6 341.5,56.9 377.6,55.8 413.8,39.6 450.0,26.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.6 88.2,123.3 124.4,107.4 160.5,101.7 196.7,97.4 232.9,82.6 269.1,72.0 305.3,62.7 341.5,57.1 377.6,43.3 413.8,26.3 450.0,18.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-div-0">div@base</label></div></figure></div>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `mul@base` D18 | 1.05 ns | 3.06 ns | 3.2 ns | 4.92 ns | 5.26 ns |
| `mul@base` D38 | 3.51 ns | 14.4 ns | 25.7 ns | 39.8 ns | 37.3 ns |
| `mul@base` D57 | 3.37 ns | 21.6 ns | 26.7 ns | 76.9 ns | 72.3 ns |
| `mul@base` D76 | 5.64 ns | 34.4 ns | 43 ns | 83.6 ns | 101 ns |
| `mul@base` D115 | 13.2 ns | 49.9 ns | 67.7 ns | 215 ns | 182 ns |
| `mul@base` D153 | 13 ns | 46.9 ns | 119 ns | 258 ns | 396 ns |
| `mul@base` D230 | 27.8 ns | 77.9 ns | 285 ns | 477 ns | 939 ns |
| `mul@base` D307 | 54.3 ns | 184 ns | 469 ns | 1.07 µs | 1.39 µs |
| `mul@base` D462 | 89.6 ns | 413 ns | 1.33 µs | 1.85 µs | 1.82 µs |
| `mul@base` D616 | 89.4 ns | 677 ns | 1.73 µs | 1.74 µs | 3.86 µs |
| `mul@base` D924 | 147 ns | 1.58 µs | 3.24 µs | 5 µs | 8.31 µs |
| `mul@base` D1232 | 192 ns | 2.22 µs | 4.73 µs | 9 µs | 13 µs |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-mul-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,209.1 88.2,188.2 124.4,188.9 160.5,179.9 196.7,165.1 232.9,165.4 269.1,152.2 305.3,140.6 341.5,131.9 377.6,131.9 413.8,123.3 450.0,118.7 450.0,45.4 413.8,53.2 377.6,66.6 341.5,79.6 305.3,84.3 269.1,91.1 232.9,106.1 196.7,119.6 160.5,129.8 124.4,135.6 88.2,147.1 52.0,181.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,209.1 88.2,188.2 124.4,188.9 160.5,179.9 196.7,165.1 232.9,165.4 269.1,152.2 305.3,140.6 341.5,131.9 377.6,131.9 413.8,123.3 450.0,118.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.5 88.2,163.7 124.4,156.6 160.5,148.5 196.7,142.1 232.9,143.2 269.1,134.3 305.3,119.4 341.5,105.4 377.6,96.8 413.8,82.1 450.0,76.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.8 88.2,153.6 124.4,152.9 160.5,144.7 196.7,136.8 232.9,127.0 269.1,111.8 305.3,103.2 341.5,85.1 377.6,80.4 413.8,69.6 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.3 88.2,146.0 124.4,134.6 160.5,133.1 196.7,116.7 232.9,113.5 269.1,102.9 305.3,88.8 341.5,79.3 377.6,80.4 413.8,62.0 450.0,51.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.2 88.2,147.1 124.4,135.6 160.5,129.8 196.7,119.6 232.9,106.1 269.1,91.1 305.3,84.3 341.5,79.6 377.6,66.6 413.8,53.2 450.0,45.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-mul-0">mul@base</label></div></figure></div>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `neg@base` D18 | 0.703 ns | 0.703 ns | 0.703 ns | 0.622 ns | 0.346 ns |
| `neg@base` D38 | 1.15 ns | 1.42 ns | 1.42 ns | 1.45 ns | 1.12 ns |
| `neg@base` D57 | 1.28 ns | 1.68 ns | 1.35 ns | 1.74 ns | 1.68 ns |
| `neg@base` D76 | 2.16 ns | 2.18 ns | 2.19 ns | 2.16 ns | 2.49 ns |
| `neg@base` D115 | 2.83 ns | 2.83 ns | 2.47 ns | 3.55 ns | 2.36 ns |
| `neg@base` D153 | 3.27 ns | 2.91 ns | 4.6 ns | 4.6 ns | 4.6 ns |
| `neg@base` D230 | 6 ns | 4.18 ns | 5.76 ns | 5.51 ns | 5.38 ns |
| `neg@base` D307 | 12.3 ns | 12.4 ns | 11 ns | 12.4 ns | 11 ns |
| `neg@base` D462 | 15 ns | 15.3 ns | 17.5 ns | 17.4 ns | 14.4 ns |
| `neg@base` D616 | 19 ns | 20.1 ns | 19.9 ns | 15.4 ns | 20.2 ns |
| `neg@base` D924 | 55.1 ns | 93.7 ns | 85.3 ns | 81.2 ns | 84.9 ns |
| `neg@base` D1232 | 47.1 ns | 68.9 ns | 68.9 ns | 70.1 ns | 61.9 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-neg-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,153.5 88.2,139.2 124.4,136.1 160.5,121.0 196.7,113.2 232.9,109.0 269.1,91.5 305.3,70.7 341.5,64.9 377.6,58.1 413.8,27.2 450.0,31.8 450.0,23.9 413.8,14.7 377.6,56.3 341.5,66.1 305.3,74.0 269.1,94.6 232.9,99.1 196.7,118.5 160.5,117.0 124.4,128.3 88.2,140.0 52.0,174.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.5 88.2,139.2 124.4,136.1 160.5,121.0 196.7,113.2 232.9,109.0 269.1,91.5 305.3,70.7 341.5,64.9 377.6,58.1 413.8,27.2 450.0,31.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,133.2 124.4,128.2 160.5,120.7 196.7,113.2 232.9,112.4 269.1,101.9 305.3,70.5 341.5,64.3 377.6,56.4 413.8,11.9 450.0,20.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,133.3 124.4,134.7 160.5,120.7 196.7,117.2 232.9,99.1 269.1,92.6 305.3,74.0 341.5,60.5 377.6,56.8 413.8,14.6 450.0,20.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,132.7 124.4,127.3 160.5,121.0 196.7,106.7 232.9,99.2 269.1,93.9 305.3,70.5 341.5,60.6 377.6,64.2 413.8,16.0 450.0,20.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.1 88.2,140.0 124.4,128.3 160.5,117.0 196.7,118.5 232.9,99.1 269.1,94.6 305.3,74.0 341.5,66.1 377.6,56.3 413.8,14.7 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-neg-0">neg@base</label></div></figure></div>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `rem@base` D18 | 2.11 ns | 2.11 ns | 2.11 ns | 2.18 ns | 2.88 ns |
| `rem@base` D38 | 2.76 ns | 3.29 ns | 3.27 ns | 3.55 ns | 2.74 ns |
| `rem@base` D57 | 4.4 ns | 7.17 ns | 6.27 ns | 8.08 ns | 7.17 ns |
| `rem@base` D76 | 9.83 ns | 8.71 ns | 8.71 ns | 9.84 ns | 8.42 ns |
| `rem@base` D115 | 12.7 ns | 12.8 ns | 8.76 ns | 14.7 ns | 7.61 ns |
| `rem@base` D153 | 15.6 ns | 12.2 ns | 20 ns | 20 ns | 20.1 ns |
| `rem@base` D230 | 32.2 ns | 19.5 ns | 28 ns | 23.5 ns | 22.6 ns |
| `rem@base` D307 | 48 ns | 48 ns | 43 ns | 47.9 ns | 42.6 ns |
| `rem@base` D462 | 74.7 ns | 72.8 ns | 89.5 ns | 84.8 ns | 68 ns |
| `rem@base` D616 | 82.7 ns | 79.8 ns | 78.9 ns | 53.8 ns | 77.2 ns |
| `rem@base` D924 | 102 ns | 105 ns | 96.6 ns | 101 ns | 90.4 ns |
| `rem@base` D1232 | 139 ns | 143 ns | 135 ns | 135 ns | 109 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-rem-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,188.4 88.2,180.6 124.4,167.1 160.5,143.8 196.7,136.3 232.9,130.5 269.1,109.5 305.3,97.9 341.5,85.1 377.6,82.2 413.8,76.1 450.0,67.1 450.0,74.1 413.8,79.6 377.6,84.1 341.5,87.8 305.3,101.3 269.1,119.7 232.9,123.2 196.7,151.3 160.5,148.3 124.4,153.0 88.2,180.8 52.0,179.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.4 88.2,180.6 124.4,167.1 160.5,143.8 196.7,136.3 232.9,130.5 269.1,109.5 305.3,97.9 341.5,85.1 377.6,82.2 413.8,76.1 450.0,67.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.4 88.2,175.5 124.4,153.0 160.5,147.3 196.7,136.3 232.9,137.5 269.1,124.0 305.3,97.9 341.5,85.9 377.6,83.2 413.8,75.2 450.0,66.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,175.7 124.4,156.8 160.5,147.3 196.7,147.2 232.9,123.2 269.1,113.5 305.3,101.1 341.5,79.9 377.6,83.5 413.8,77.7 450.0,67.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,173.3 124.4,149.5 160.5,143.8 196.7,132.1 232.9,123.2 269.1,118.6 305.3,98.0 341.5,81.5 377.6,94.6 413.8,76.4 450.0,68.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.4 88.2,180.8 124.4,153.0 160.5,148.3 196.7,151.3 232.9,123.2 269.1,119.7 305.3,101.3 341.5,87.8 377.6,84.1 413.8,79.6 450.0,74.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-rem-0">rem@base</label></div></figure></div>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Row | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| `sub@base` D18 | 1.05 ns | 1.05 ns | 1.05 ns | 1.24 ns | 1.05 ns |
| `sub@base` D38 | 1.44 ns | 1.62 ns | 1.62 ns | 1.82 ns | 1.4 ns |
| `sub@base` D57 | 1.53 ns | 2.27 ns | 1.94 ns | 2.51 ns | 2.27 ns |
| `sub@base` D76 | 3.45 ns | 3.09 ns | 3.1 ns | 3.45 ns | 3.09 ns |
| `sub@base` D115 | 4.85 ns | 4.86 ns | 4.2 ns | 5.56 ns | 3.75 ns |
| `sub@base` D153 | 6.6 ns | 6.37 ns | 8.48 ns | 8.45 ns | 8.48 ns |
| `sub@base` D230 | 16.2 ns | 13.1 ns | 13.7 ns | 15.5 ns | 13.7 ns |
| `sub@base` D307 | 25.2 ns | 25.2 ns | 23.5 ns | 25.2 ns | 23.5 ns |
| `sub@base` D462 | 38.2 ns | 36.9 ns | 43.1 ns | 43.4 ns | 35.8 ns |
| `sub@base` D616 | 45.4 ns | 45.2 ns | 44.6 ns | 37.5 ns | 45.1 ns |
| `sub@base` D924 | 76.6 ns | 85 ns | 84.9 ns | 79.8 ns | 84.7 ns |
| `sub@base` D1232 | 95.9 ns | 106 ns | 103 ns | 106 ns | 95.3 ns |

<div class="perf-chart"><input type="checkbox" class="fam-toggle" id="f-sub-0" checked><figure><svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><g class="fam-1"><polygon points="52.0,208.5 88.2,199.4 124.4,197.6 160.5,174.1 196.7,164.3 232.9,155.3 269.1,129.3 305.3,116.6 341.5,104.5 377.6,99.5 413.8,84.4 450.0,77.9 450.0,78.1 413.8,81.5 377.6,99.7 341.5,106.4 305.3,118.6 269.1,134.3 232.9,148.1 196.7,171.7 160.5,177.3 124.4,186.3 88.2,200.3 52.0,208.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,208.5 88.2,199.4 124.4,197.6 160.5,174.1 196.7,164.3 232.9,155.3 269.1,129.3 305.3,116.6 341.5,104.5 377.6,99.5 413.8,84.4 450.0,77.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,208.5 88.2,196.1 124.4,186.3 160.5,177.3 196.7,164.2 232.9,156.4 269.1,135.5 305.3,116.6 341.5,105.6 377.6,99.7 413.8,81.4 450.0,75.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,208.4 88.2,196.1 124.4,190.8 160.5,177.3 196.7,168.4 232.9,148.1 269.1,134.3 305.3,118.6 341.5,101.0 377.6,100.1 413.8,81.4 450.0,75.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,203.7 88.2,192.6 124.4,183.4 160.5,174.1 196.7,160.3 232.9,148.2 269.1,130.7 305.3,116.6 341.5,100.9 377.6,105.1 413.8,83.2 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,208.6 88.2,200.3 124.4,186.3 160.5,177.3 196.7,171.7 232.9,148.1 269.1,134.3 305.3,118.6 341.5,106.4 377.6,99.7 413.8,81.5 450.0,78.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/></g><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg><figcaption>Median time vs width (log scale). solid = scale 0 and max, dashed = the intermediate band-edge scales.</figcaption><div class="fam-legend"><label class="fam-key k1" for="f-sub-0">sub@base</label></div></figure></div>

</div>
<!-- END GENERATED:performance:body:arithmetic -->
