# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 131 ns | 132 ns | 154 ns | 392 ns | 392 ns |
| D38 | 251 ns | 241 ns | 398 ns | 598 ns | 835 ns |
| D57 | 228 ns | 587 ns | 530 ns | 1.04 µs | 1.15 µs |
| D76 | 443 ns | 785 ns | 1.15 µs | 1.06 µs | 1.25 µs |
| D115 | 227 ns | 764 ns | 1.45 µs | 2.85 µs | 5.19 µs |
| D153 | 245 ns | 860 ns | 2.07 µs | 5.63 µs | 6.94 µs |
| D230 | 280 ns | 1.41 µs | 4.69 µs | 10.1 µs | 14.1 µs |
| D307 | 317 ns | 1.79 µs | 11.5 µs | 14 µs | 20.6 µs |
| D462 | 381 ns | 3.36 µs | 24.1 µs | 32.9 µs | 47.8 µs |
| D616 | 429 ns | 3.79 µs | 36.8 µs | 49 µs | 76.9 µs |
| D924 | 551 ns | 8.09 µs | 89.5 µs | 76.3 µs | 180 µs |
| D1232 | 1.14 µs | 16.1 µs | 138 µs | 197 µs | 264 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,204.2 88.2,190.0 124.4,192.1 160.5,177.7 196.7,192.2 232.9,190.6 269.1,187.6 305.3,184.9 341.5,181.0 377.6,178.4 413.8,172.9 450.0,157.1 450.0,39.0 413.8,47.2 377.6,65.7 341.5,76.0 305.3,94.3 269.1,102.5 232.9,117.9 196.7,124.2 160.5,155.2 124.4,157.0 88.2,163.9 52.0,180.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,204.2 88.2,190.0 124.4,192.1 160.5,177.7 196.7,192.2 232.9,190.6 269.1,187.6 305.3,184.9 341.5,181.0 377.6,178.4 413.8,172.9 450.0,157.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,203.9 88.2,190.9 124.4,171.6 160.5,165.3 196.7,165.8 232.9,163.3 269.1,152.5 305.3,147.4 341.5,133.7 377.6,131.1 413.8,114.6 450.0,99.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,200.6 88.2,180.0 124.4,173.8 160.5,157.0 196.7,151.9 232.9,144.2 269.1,126.4 305.3,106.9 341.5,90.9 377.6,81.7 413.8,62.4 450.0,53.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.3 88.2,171.2 124.4,159.1 160.5,158.7 196.7,137.3 232.9,122.5 269.1,109.7 305.3,102.7 341.5,84.1 377.6,75.5 413.8,65.9 450.0,45.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.3 88.2,163.9 124.4,157.0 160.5,155.2 196.7,124.2 232.9,117.9 269.1,102.5 305.3,94.3 341.5,76.0 377.6,65.7 413.8,47.2 450.0,39.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 1.62 µs | 3.19 µs | 3.42 µs | 3.63 µs |
| D38 | 1.76 ns | 2.79 µs | 3.17 µs | 4.31 µs | 4.77 µs |
| D57 | 1.8 ns | 3.82 µs | 3.69 µs | 6.18 µs | 9.2 µs |
| D76 | 3.43 ns | 5.99 µs | 7.73 µs | 7.86 µs | 8.16 µs |
| D115 | 13.2 ns | 7 µs | 8.38 µs | 15.8 µs | 22.5 µs |
| D153 | 16.2 ns | 7.24 µs | 10.1 µs | 22.1 µs | 38.4 µs |
| D230 | 39.7 ns | 13.2 µs | 13.1 µs | 42.6 µs | 78.9 µs |
| D307 | 85.4 ns | 15.9 µs | 38.9 µs | 67.4 µs | 117 µs |
| D462 | 116 ns | 24 µs | 80.4 µs | 155 µs | 232 µs |
| D616 | 109 ns | 31.1 µs | 130 µs | 232 µs | 412 µs |
| D924 | 185 ns | 68.1 µs | 287 µs | 382 µs | 913 µs |
| D1232 | 371 ns | 140 µs | 409 µs | 902 µs | 2.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,203.0 124.4,202.7 160.5,194.7 196.7,177.9 232.9,175.4 269.1,164.3 305.3,154.8 341.5,151.0 377.6,151.8 413.8,145.2 450.0,136.6 450.0,27.2 413.8,39.7 377.6,49.6 341.5,56.7 305.3,65.2 269.1,70.1 232.9,79.0 196.7,85.6 160.5,98.2 124.4,96.7 88.2,104.9 52.0,108.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,203.0 124.4,202.7 160.5,194.7 196.7,177.9 232.9,175.4 269.1,164.3 305.3,154.8 341.5,151.0 377.6,151.8 413.8,145.2 450.0,136.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.3 88.2,111.5 124.4,107.7 160.5,102.1 196.7,100.1 232.9,99.7 269.1,92.3 305.3,90.0 341.5,84.9 377.6,81.6 413.8,71.9 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,110.0 124.4,108.1 160.5,98.9 196.7,97.9 232.9,95.6 269.1,92.4 305.3,78.9 341.5,69.8 377.6,63.9 413.8,54.1 450.0,49.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,106.2 124.4,101.7 160.5,98.7 196.7,90.0 232.9,85.9 269.1,77.7 305.3,72.0 341.5,61.7 377.6,56.7 413.8,50.5 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,104.9 124.4,96.7 160.5,98.2 196.7,85.6 232.9,79.0 269.1,70.1 305.3,65.2 341.5,56.7 377.6,49.6 413.8,39.7 450.0,27.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17 ns | 41.4 ns | 40.8 ns | 41.1 ns | 41.4 ns |
| D38 | 15.4 ns | 37.3 ns | 66 ns | 67.2 ns | 95.2 ns |
| D57 | 11.8 ns | 41.1 ns | 67.9 ns | 398 ns | 390 ns |
| D76 | 18.2 ns | 69.1 ns | 391 ns | 321 ns | 527 ns |
| D115 | 20.1 ns | 80.4 ns | 274 ns | 641 ns | 976 ns |
| D153 | 25.6 ns | 400 ns | 428 ns | 968 ns | 1.7 µs |
| D230 | 29.3 ns | 426 ns | 648 ns | 1.85 µs | 2.81 µs |
| D307 | 43.7 ns | 688 ns | 1.75 µs | 2.32 µs | 4.93 µs |
| D462 | 69.1 ns | 1.23 µs | 3.16 µs | 5.69 µs | 7.22 µs |
| D616 | 65.7 ns | 1.66 µs | 5.44 µs | 9.27 µs | 13 µs |
| D924 | 109 ns | 3.03 µs | 11 µs | 14.9 µs | 23.5 µs |
| D1232 | 100 ns | 6.04 µs | 18.7 µs | 24.7 µs | 43.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,200.6 124.4,206.5 160.5,197.0 196.7,194.9 232.9,189.6 269.1,186.6 305.3,178.0 341.5,168.0 377.6,169.1 413.8,158.2 450.0,160.0 450.0,28.3 413.8,41.4 377.6,54.4 341.5,67.1 305.3,75.4 269.1,87.5 232.9,98.5 196.7,110.5 160.5,123.9 124.4,130.4 88.2,161.1 52.0,179.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,200.6 124.4,206.5 160.5,197.0 196.7,194.9 232.9,189.6 269.1,186.6 305.3,178.0 341.5,168.0 377.6,169.1 413.8,158.2 450.0,160.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.1 88.2,181.4 124.4,179.3 160.5,168.0 196.7,164.7 232.9,129.9 269.1,128.5 305.3,118.1 341.5,105.4 377.6,98.9 413.8,85.9 450.0,71.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,169.0 124.4,168.4 160.5,130.4 196.7,138.1 232.9,128.4 269.1,119.4 305.3,97.8 341.5,85.0 377.6,73.2 413.8,57.9 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.3 88.2,168.6 124.4,130.0 160.5,134.7 196.7,119.6 232.9,110.7 269.1,96.6 305.3,91.8 341.5,72.2 377.6,61.6 413.8,51.3 450.0,40.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.1 88.2,161.1 124.4,130.4 160.5,123.9 196.7,110.5 232.9,98.5 269.1,87.5 305.3,75.4 341.5,67.1 377.6,54.4 413.8,41.4 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 271 ns | 374 ns | 376 ns | 383 ns |
| D38 | 2.11 ns | 365 ns | 358 ns | 383 ns | 394 ns |
| D57 | 208 ns | 484 ns | 419 ns | 515 ns | 626 ns |
| D76 | 280 ns | 486 ns | 509 ns | 536 ns | 408 ns |
| D115 | 318 ns | 522 ns | 365 ns | 958 ns | 991 ns |
| D153 | 303 ns | 523 ns | 368 ns | 1 µs | 1.45 µs |
| D230 | 515 ns | 661 ns | 640 ns | 1.3 µs | 1.85 µs |
| D307 | 769 ns | 692 ns | 1.2 µs | 1.31 µs | 10.6 µs |
| D462 | 1.19 µs | 3.12 µs | 3.27 µs | 3.97 µs | 4.99 µs |
| D616 | 1.23 µs | 1.4 µs | 1.82 µs | 2.47 µs | 3.69 µs |
| D924 | 2.02 µs | 1.76 µs | 3.14 µs | 2.38 µs | 4.81 µs |
| D1232 | 3.05 µs | 3.2 µs | 4.31 µs | 4.29 µs | 6.35 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.2 88.2,197.0 124.4,117.3 160.5,112.1 196.7,109.9 232.9,110.7 269.1,101.5 305.3,94.6 341.5,87.0 377.6,86.5 413.8,77.8 450.0,70.6 450.0,57.9 413.8,62.7 377.6,67.3 341.5,62.1 305.3,49.1 269.1,79.3 232.9,83.5 196.7,90.2 160.5,105.6 124.4,98.1 88.2,106.2 52.0,106.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.2 88.2,197.0 124.4,117.3 160.5,112.1 196.7,109.9 232.9,110.7 269.1,101.5 305.3,94.6 341.5,87.0 377.6,86.5 413.8,77.8 450.0,70.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.7 88.2,107.5 124.4,102.6 160.5,102.5 196.7,101.3 232.9,101.2 269.1,97.2 305.3,96.4 341.5,70.2 377.6,84.1 413.8,80.2 450.0,69.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,107.9 124.4,105.1 160.5,101.7 196.7,107.5 232.9,107.4 269.1,97.7 305.3,86.8 341.5,69.4 377.6,79.6 413.8,70.1 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,106.7 124.4,101.5 160.5,100.8 196.7,90.7 232.9,89.9 269.1,85.4 305.3,85.3 341.5,66.1 377.6,74.3 413.8,74.9 450.0,64.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,106.2 124.4,98.1 160.5,105.6 196.7,90.2 232.9,83.5 269.1,79.3 305.3,49.1 341.5,62.1 377.6,67.3 413.8,62.7 450.0,57.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.74 µs | 3.09 µs | 5.78 µs | 6.31 µs | 6.97 µs |
| D38 | 2.89 µs | 5.22 µs | 6.23 µs | 8.6 µs | 9.99 µs |
| D57 | 1.65 µs | 2.59 µs | 2.11 µs | 3 µs | 2.87 µs |
| D76 | 2.51 µs | 2.64 µs | 2.98 µs | 2.47 µs | 1.95 µs |
| D115 | 4.71 µs | 5.28 µs | 3.24 µs | 4.88 µs | 5.73 µs |
| D153 | 4.48 µs | 5 µs | 3.39 µs | 5.82 µs | 6.72 µs |
| D230 | 6.21 µs | 7.18 µs | 4.83 µs | 8.8 µs | 9.82 µs |
| D307 | 9.8 µs | 10.9 µs | 13.7 µs | 11.9 µs | 15.7 µs |
| D462 | 9.77 µs | 13 µs | 14.2 µs | 16.4 µs | 18.3 µs |
| D616 | 12.2 µs | 20.1 µs | 29.6 µs | 31.2 µs | 41.5 µs |
| D924 | 24 µs | 34.8 µs | 59.7 µs | 46.7 µs | 85.1 µs |
| D1232 | 33.3 µs | 66.2 µs | 89.3 µs | 106 µs | 125 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.8 88.2,179.2 124.4,195.5 160.5,183.3 196.7,165.1 232.9,166.6 269.1,157.1 305.3,143.9 341.5,144.0 377.6,137.6 413.8,118.0 450.0,108.5 450.0,70.1 413.8,81.4 377.6,102.1 341.5,125.8 305.3,130.3 269.1,143.8 232.9,154.8 196.7,159.5 160.5,190.7 124.4,179.5 88.2,143.4 52.0,153.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.8 88.2,179.2 124.4,195.5 160.5,183.3 196.7,165.1 232.9,166.6 269.1,157.1 305.3,143.9 341.5,144.0 377.6,137.6 413.8,118.0 450.0,108.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,177.3 88.2,162.2 124.4,182.5 160.5,181.9 196.7,161.8 232.9,163.4 269.1,152.9 305.3,140.8 341.5,135.7 377.6,123.1 413.8,107.2 450.0,88.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.2 88.2,157.0 124.4,188.3 160.5,178.4 196.7,176.0 232.9,174.6 269.1,164.4 305.3,134.3 341.5,133.2 377.6,111.9 413.8,91.6 450.0,80.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.7 88.2,147.7 124.4,178.2 160.5,183.9 196.7,164.1 232.9,159.0 269.1,147.0 305.3,138.3 341.5,129.0 377.6,110.4 413.8,98.7 450.0,75.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.8 88.2,143.4 124.4,179.5 160.5,190.7 196.7,159.5 232.9,154.8 269.1,143.8 305.3,130.3 341.5,125.8 377.6,102.1 413.8,81.4 450.0,70.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.72 ns | 2.06 µs | 3.85 µs | 4.04 µs | 4.29 µs |
| D38 | 6.57 ns | 3.42 µs | 3.8 µs | 4.93 µs | 5.43 µs |
| D57 | 59.2 ns | 4.2 µs | 3.78 µs | 5.1 µs | 5.14 µs |
| D76 | 74.8 ns | 4.12 µs | 5.27 µs | 4.38 µs | 4.26 µs |
| D115 | 147 ns | 8.62 µs | 5.67 µs | 9.07 µs | 10.8 µs |
| D153 | 177 ns | 8.53 µs | 6.15 µs | 10.9 µs | 13.1 µs |
| D230 | 282 ns | 12.6 µs | 9.05 µs | 18.3 µs | 21.4 µs |
| D307 | 444 ns | 19.3 µs | 23.1 µs | 23.9 µs | 32.1 µs |
| D462 | 646 ns | 77.7 µs | 138 µs | 207 µs | 265 µs |
| D616 | 605 ns | 151 µs | 313 µs | 314 µs | 531 µs |
| D924 | 906 ns | 375 µs | 487 µs | 523 µs | 1.63 ms |
| D1232 | 1.46 µs | 830 µs | 800 µs | 2.19 ms | 2.47 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,186.6 124.4,159.4 160.5,156.5 196.7,148.1 232.9,145.8 269.1,140.0 305.3,134.4 341.5,129.7 377.6,130.5 413.8,125.5 450.0,119.6 450.0,27.3 413.8,32.5 377.6,46.4 341.5,55.0 305.3,81.2 269.1,86.2 232.9,92.3 196.7,94.8 160.5,106.3 124.4,104.0 88.2,103.3 52.0,106.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,186.6 124.4,159.4 160.5,156.5 196.7,148.1 232.9,145.8 269.1,140.0 305.3,134.4 341.5,129.7 377.6,130.5 413.8,125.5 450.0,119.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.3 88.2,109.0 124.4,106.5 160.5,106.7 196.7,97.6 232.9,97.7 269.1,92.8 305.3,87.6 341.5,70.3 377.6,62.0 413.8,50.7 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,107.7 124.4,107.8 160.5,103.7 196.7,102.8 232.9,101.7 269.1,97.0 305.3,85.3 341.5,63.2 377.6,53.0 413.8,47.5 450.0,41.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,104.5 124.4,104.1 160.5,106.0 196.7,96.9 232.9,94.7 269.1,88.2 305.3,84.9 341.5,58.1 377.6,53.0 413.8,46.6 450.0,28.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,103.3 124.4,104.0 160.5,106.3 196.7,94.8 232.9,92.3 269.1,86.2 305.3,81.2 341.5,55.0 377.6,46.4 413.8,32.5 450.0,27.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.91 ns | 13.5 ns | 18.5 ns | 28.6 ns | 28.9 ns |
| D38 | 7.28 ns | 21.2 ns | 30.3 ns | 419 ns | 617 ns |
| D57 | 144 ns | 208 ns | 317 ns | 676 ns | 770 ns |
| D76 | 191 ns | 267 ns | 679 ns | 565 ns | 725 ns |
| D115 | 95 ns | 610 ns | 529 ns | 1.15 µs | 1.68 µs |
| D153 | 106 ns | 1.1 µs | 849 ns | 1.88 µs | 2.63 µs |
| D230 | 135 ns | 434 ns | 1.34 µs | 3.42 µs | 4.29 µs |
| D307 | 159 ns | 2.37 µs | 3.7 µs | 3.93 µs | 6.94 µs |
| D462 | 186 ns | 3.62 µs | 5.82 µs | 9.35 µs | 11.8 µs |
| D616 | 232 ns | 1.58 µs | 10.5 µs | 11.5 µs | 19.8 µs |
| D924 | 256 ns | 2.47 µs | 16.9 µs | 16.5 µs | 32.5 µs |
| D1232 | 342 ns | 16.3 µs | 27.3 µs | 35.4 µs | 52.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.1 88.2,175.5 124.4,123.6 160.5,118.8 196.7,130.9 232.9,129.0 269.1,124.8 305.3,122.0 341.5,119.2 377.6,115.4 413.8,113.7 450.0,108.6 450.0,21.2 413.8,29.5 377.6,38.2 341.5,47.2 305.3,56.3 269.1,64.7 232.9,73.2 196.7,81.0 160.5,95.6 124.4,94.5 88.2,98.4 52.0,151.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.1 88.2,175.5 124.4,123.6 160.5,118.8 196.7,130.9 232.9,129.0 269.1,124.8 305.3,122.0 341.5,119.2 377.6,115.4 413.8,113.7 450.0,108.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.8 88.2,156.9 124.4,117.2 160.5,112.9 196.7,98.6 232.9,88.4 269.1,104.5 305.3,75.0 341.5,67.7 377.6,82.1 413.8,74.3 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,150.8 124.4,110.0 160.5,96.7 196.7,101.1 232.9,92.8 269.1,85.0 305.3,67.3 341.5,59.4 377.6,49.2 413.8,40.9 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.7 88.2,105.1 124.4,96.8 160.5,99.9 196.7,87.5 232.9,79.1 269.1,68.7 305.3,66.2 341.5,51.2 377.6,47.5 413.8,41.3 450.0,28.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.5 88.2,98.4 124.4,94.5 160.5,95.6 196.7,81.0 232.9,73.2 269.1,64.7 305.3,56.3 341.5,47.2 377.6,38.2 413.8,29.5 450.0,21.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
