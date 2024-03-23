const createGrid = (rows: number, cols: number = 3, bottomSpace: number = 0, gutterX: number = 1, gutterY: number = 2) => {
    const grids = [];
    const width = Math.floor((100 - (cols + 1) * gutterX) / cols);
    const height = Math.floor((100 - bottomSpace - (rows + 1) * gutterY) / rows);

    for (let col = 0; col < cols; col++) {
        for (let row = 0; row < rows; row++) {
            const left = col * width + (col + 1) * gutterX;
            const top = row * height + (row + 1) * gutterY;
            grids.push({
                show: true,
                topCenter: {left: `${left}%`, top: `${top - gutterY / 2}%`},
                left: `${left}%`,
                top: `${top}%`,
                height: `${height}%`,
                width: `${width}%`,
                containLabel: true,
            });
        }
    }
    return grids;
}

export const sensorChartInit = (titles: string[], rows: number, cols: number) => {
    const grid = createGrid(rows, cols)
    const xAxis = []
    const yAxis = []
    const series = []
    const title = []

    for (let i = 0; i < grid.length; i++) {
        title.push({
            text: titles[i],
            left: grid[i].topCenter.left,
            top: grid[i].topCenter.top,
            textStyle: {
                width: 192,
                height: 12,
                fontSize: 12,
                fontFamily: 'HouschkaRounded',
                overflow: 'truncate'
            },
            backgroundColor: 'white',
            borderWidth: 1,
            borderRadius: [6, 6, 6, 6],
        })
        xAxis.push({
            gridIndex: i,
            type: 'value',
            silent: true,
            show: false,
            axisLabel: {
                show: false
            }
        })
        yAxis.push({
            gridIndex: i,
            type: 'value',
            silent: true,
            scale: true,
            axisLabel: {
                margin: 3,
                showMinLabel: false,
                showMaxLabel: false,
            },
            axisTick: {
                show: false
            },
            animation: false
        })
        series.push({
            name: Math.floor(i / rows),
            type: 'line',
            showSymbol: false,
            animation: false,
            smooth: true,
            seriesLayoutBy: 'row',
            sampling: 'lttb',
            xAxisIndex: i,
            yAxisIndex: i,
            encode: {
                x: grid.length,
                y: i
            },
            silent: true
        })
    }


    return {
        title,
        xAxis,
        yAxis,
        grid,
        series,
    }
}
