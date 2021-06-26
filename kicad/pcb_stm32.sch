EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev "1"
Comp ""
Comment1 "Designed for AISLER 2-Layer Service"
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L power:GND #PWR02
U 1 1 60D7247E
P 1650 2900
F 0 "#PWR02" H 1650 2650 50  0001 C CNN
F 1 "GND" V 1655 2772 50  0000 R CNN
F 2 "" H 1650 2900 50  0001 C CNN
F 3 "" H 1650 2900 50  0001 C CNN
	1    1650 2900
	0    -1   -1   0   
$EndComp
$Comp
L MCU_Module:Arduino_Nano_v2.x A1
U 1 1 60D74315
P 4800 3100
F 0 "A1" H 4800 2011 50  0000 C CNN
F 1 "Arduino_Nano_v2.x" H 4800 1920 50  0000 C CNN
F 2 "Module:Arduino_Nano" H 4800 3100 50  0001 C CIN
F 3 "https://www.arduino.cc/en/uploads/Main/ArduinoNanoManual23.pdf" H 4800 3100 50  0001 C CNN
	1    4800 3100
	1    0    0    -1  
$EndComp
$Comp
L Device:R R1
U 1 1 60D755ED
P 2200 2800
F 0 "R1" V 2000 2800 50  0000 C CNN
F 1 "10K" V 2100 2800 50  0000 C CNN
F 2 "Resistor_SMD:R_0603_1608Metric" V 2130 2800 50  0001 C CNN
F 3 "~" H 2200 2800 50  0001 C CNN
	1    2200 2800
	0    1    1    0   
$EndComp
$Comp
L Device:R R2
U 1 1 60D759A4
P 2200 3700
F 0 "R2" V 2000 3700 50  0000 C CNN
F 1 "100K" V 2100 3700 50  0000 C CNN
F 2 "Resistor_SMD:R_0603_1608Metric" V 2130 3700 50  0001 C CNN
F 3 "~" H 2200 3700 50  0001 C CNN
	1    2200 3700
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x02_Male J1
U 1 1 60D75FFC
P 1450 2800
F 0 "J1" H 1550 3050 50  0000 C CNN
F 1 "PRESET" H 1550 2950 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x02_P2.54mm_Vertical" H 1450 2800 50  0001 C CNN
F 3 "~" H 1450 2800 50  0001 C CNN
	1    1450 2800
	1    0    0    -1  
$EndComp
$Comp
L Connector:Conn_01x02_Male J2
U 1 1 60D767C0
P 1450 3700
F 0 "J2" H 1550 3950 50  0000 C CNN
F 1 "PROBE" H 1550 3850 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x02_P2.54mm_Vertical" H 1450 3700 50  0001 C CNN
F 3 "~" H 1450 3700 50  0001 C CNN
	1    1450 3700
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR03
U 1 1 60D76DD4
P 1650 3800
F 0 "#PWR03" H 1650 3550 50  0001 C CNN
F 1 "GND" V 1655 3672 50  0000 R CNN
F 2 "" H 1650 3800 50  0001 C CNN
F 3 "" H 1650 3800 50  0001 C CNN
	1    1650 3800
	0    -1   -1   0   
$EndComp
$Comp
L power:+3V3 #PWR04
U 1 1 60D78E20
P 2500 2800
F 0 "#PWR04" H 2500 2650 50  0001 C CNN
F 1 "+3V3" V 2500 2950 50  0000 L CNN
F 2 "" H 2500 2800 50  0001 C CNN
F 3 "" H 2500 2800 50  0001 C CNN
	1    2500 2800
	0    1    1    0   
$EndComp
$Comp
L power:+3V3 #PWR05
U 1 1 60D794CC
P 2500 3700
F 0 "#PWR05" H 2500 3550 50  0001 C CNN
F 1 "+3V3" V 2500 3850 50  0000 L CNN
F 2 "" H 2500 3700 50  0001 C CNN
F 3 "" H 2500 3700 50  0001 C CNN
	1    2500 3700
	0    1    1    0   
$EndComp
Wire Wire Line
	2350 2800 2500 2800
Wire Wire Line
	1650 2800 1900 2800
Wire Wire Line
	1650 3700 1900 3700
Text GLabel 1900 2650 1    50   Input ~ 0
ADC1
Wire Wire Line
	1900 2650 1900 2800
Connection ~ 1900 2800
Wire Wire Line
	1900 2800 2050 2800
Text GLabel 1900 3550 1    50   Input ~ 0
ADC2
Wire Wire Line
	1900 3550 1900 3700
Connection ~ 1900 3700
Wire Wire Line
	1900 3700 2050 3700
Wire Wire Line
	2350 3700 2500 3700
$Comp
L power:GND #PWR01
U 1 1 60D7BD2F
P 1300 1200
F 0 "#PWR01" H 1300 950 50  0001 C CNN
F 1 "GND" H 1300 1000 50  0000 C CNN
F 2 "" H 1300 1200 50  0001 C CNN
F 3 "" H 1300 1200 50  0001 C CNN
	1    1300 1200
	1    0    0    -1  
$EndComp
$Comp
L power:VCC #PWR06
U 1 1 60D7C277
P 2650 1200
F 0 "#PWR06" H 2650 1050 50  0001 C CNN
F 1 "VCC" H 2650 1400 50  0000 C CNN
F 2 "" H 2650 1200 50  0001 C CNN
F 3 "" H 2650 1200 50  0001 C CNN
	1    2650 1200
	-1   0    0    1   
$EndComp
$Comp
L Device:Battery BT1
U 1 1 60D7CE5C
P 1700 1200
F 0 "BT1" V 1500 1200 50  0000 C CNN
F 1 "Battery" V 1400 1200 50  0000 C CNN
F 2 "TerminalBlock_Phoenix:TerminalBlock_Phoenix_MKDS-1,5-2-5.08_1x02_P5.08mm_Horizontal" V 1700 1260 50  0001 C CNN
F 3 "~" V 1700 1260 50  0001 C CNN
	1    1700 1200
	0    -1   -1   0   
$EndComp
$Comp
L Switch:SW_SPST SW1
U 1 1 60D7E250
P 2250 1200
F 0 "SW1" H 2250 1435 50  0000 C CNN
F 1 "SW_SPST" H 2250 1344 50  0000 C CNN
F 2 "TerminalBlock_Phoenix:TerminalBlock_Phoenix_MKDS-1,5-2-5.08_1x02_P5.08mm_Horizontal" H 2250 1200 50  0001 C CNN
F 3 "~" H 2250 1200 50  0001 C CNN
	1    2250 1200
	1    0    0    -1  
$EndComp
Wire Wire Line
	2450 1200 2500 1200
Wire Wire Line
	2050 1200 1950 1200
Wire Wire Line
	1500 1200 1450 1200
$Comp
L power:PWR_FLAG #FLG01
U 1 1 60D7F112
P 1450 1200
F 0 "#FLG01" H 1450 1275 50  0001 C CNN
F 1 "PWR_FLAG" H 1450 1373 50  0001 C CNN
F 2 "" H 1450 1200 50  0001 C CNN
F 3 "~" H 1450 1200 50  0001 C CNN
	1    1450 1200
	1    0    0    -1  
$EndComp
$Comp
L power:PWR_FLAG #FLG02
U 1 1 60D7FC1C
P 1950 1200
F 0 "#FLG02" H 1950 1275 50  0001 C CNN
F 1 "PWR_FLAG" H 1950 1373 50  0001 C CNN
F 2 "" H 1950 1200 50  0001 C CNN
F 3 "~" H 1950 1200 50  0001 C CNN
	1    1950 1200
	1    0    0    -1  
$EndComp
Connection ~ 1950 1200
Wire Wire Line
	1950 1200 1900 1200
$Comp
L power:PWR_FLAG #FLG03
U 1 1 60D7FF71
P 2500 1200
F 0 "#FLG03" H 2500 1275 50  0001 C CNN
F 1 "PWR_FLAG" H 2500 1373 50  0001 C CNN
F 2 "" H 2500 1200 50  0001 C CNN
F 3 "~" H 2500 1200 50  0001 C CNN
	1    2500 1200
	1    0    0    -1  
$EndComp
Connection ~ 2500 1200
Wire Wire Line
	2500 1200 2650 1200
Connection ~ 1450 1200
Wire Wire Line
	1300 1200 1450 1200
$Comp
L Connector:Conn_01x04_Male J4
U 1 1 60D81584
P 7500 3150
F 0 "J4" H 7650 3400 50  0000 R CNN
F 1 "DISP_PORT" H 7800 3500 50  0000 R CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x04_P2.54mm_Vertical" H 7500 3150 50  0001 C CNN
F 3 "~" H 7500 3150 50  0001 C CNN
	1    7500 3150
	-1   0    0    1   
$EndComp
$Comp
L power:VCC #PWR07
U 1 1 60D828A2
P 4700 2100
F 0 "#PWR07" H 4700 1950 50  0001 C CNN
F 1 "VCC" H 4700 2300 50  0000 C CNN
F 2 "" H 4700 2100 50  0001 C CNN
F 3 "" H 4700 2100 50  0001 C CNN
	1    4700 2100
	1    0    0    -1  
$EndComp
$Comp
L Connector_Generic:Conn_02x04_Odd_Even J3
U 1 1 60D8576A
P 7300 2300
F 0 "J3" H 7350 2650 50  0000 C CNN
F 1 "ESP-01S" H 7350 2550 50  0000 C CNN
F 2 "Connector_PinSocket_2.54mm:PinSocket_2x04_P2.54mm_Horizontal" H 7300 2300 50  0001 C CNN
F 3 "~" H 7300 2300 50  0001 C CNN
	1    7300 2300
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR013
U 1 1 60D86940
P 7600 2200
F 0 "#PWR013" H 7600 1950 50  0001 C CNN
F 1 "GND" V 7600 2050 50  0000 R CNN
F 2 "" H 7600 2200 50  0001 C CNN
F 3 "" H 7600 2200 50  0001 C CNN
	1    7600 2200
	0    -1   -1   0   
$EndComp
Text GLabel 7100 2200 0    50   Input ~ 0
ESP_TX
$Comp
L power:+3V3 #PWR010
U 1 1 60D87DEC
P 7100 2500
F 0 "#PWR010" H 7100 2350 50  0001 C CNN
F 1 "+3V3" V 7100 2650 50  0000 L CNN
F 2 "" H 7100 2500 50  0001 C CNN
F 3 "" H 7100 2500 50  0001 C CNN
	1    7100 2500
	0    -1   -1   0   
$EndComp
Text GLabel 7600 2500 2    50   Input ~ 0
ESP_RX
Text GLabel 7100 2300 0    50   Input ~ 0
ESP_CH_PD
Text GLabel 7600 2300 2    50   Input ~ 0
ESP_GPIO2
Text GLabel 7100 2400 0    50   Input ~ 0
ESP_RST
Text GLabel 7600 2400 2    50   Input ~ 0
ESP_GIO0
Text GLabel 7300 2950 0    50   BiDi ~ 0
DPY_SDA
Text GLabel 7300 3050 0    50   BiDi ~ 0
DPY_SCK
$Comp
L power:+3V3 #PWR011
U 1 1 60D89377
P 7300 3150
F 0 "#PWR011" H 7300 3000 50  0001 C CNN
F 1 "+3V3" V 7300 3300 50  0000 L CNN
F 2 "" H 7300 3150 50  0001 C CNN
F 3 "" H 7300 3150 50  0001 C CNN
	1    7300 3150
	0    -1   -1   0   
$EndComp
$Comp
L power:GND #PWR012
U 1 1 60D89803
P 7300 3250
F 0 "#PWR012" H 7300 3000 50  0001 C CNN
F 1 "GND" V 7300 3100 50  0000 R CNN
F 2 "" H 7300 3250 50  0001 C CNN
F 3 "" H 7300 3250 50  0001 C CNN
	1    7300 3250
	0    1    1    0   
$EndComp
$Comp
L power:+3V3 #PWR08
U 1 1 60D8A736
P 4900 2100
F 0 "#PWR08" H 4900 1950 50  0001 C CNN
F 1 "+3V3" H 4900 2300 50  0000 C CNN
F 2 "" H 4900 2100 50  0001 C CNN
F 3 "" H 4900 2100 50  0001 C CNN
	1    4900 2100
	1    0    0    -1  
$EndComp
Text GLabel 4300 2500 0    50   Input ~ 0
ESP_TX
Text GLabel 4300 2600 0    50   Input ~ 0
ESP_RX
Text GLabel 5300 3100 2    50   Input ~ 0
ADC1
Text GLabel 5300 3200 2    50   Input ~ 0
ADC2
Text GLabel 5300 3600 2    50   BiDi ~ 0
DPY_SCK
Text GLabel 5300 3500 2    50   BiDi ~ 0
DPY_SDA
NoConn ~ 5300 2600
NoConn ~ 5300 2500
NoConn ~ 4300 2900
NoConn ~ 4300 3000
NoConn ~ 4300 3200
NoConn ~ 5000 2100
NoConn ~ 4800 4100
NoConn ~ 4900 4100
NoConn ~ 4300 3800
NoConn ~ 4300 3700
NoConn ~ 4300 3500
NoConn ~ 4300 3300
NoConn ~ 5300 3300
NoConn ~ 5300 3400
NoConn ~ 5300 3700
NoConn ~ 5300 3800
NoConn ~ 4300 3600
$Comp
L power:+3V3 #PWR09
U 1 1 60D9235F
P 5300 2900
F 0 "#PWR09" H 5300 2750 50  0001 C CNN
F 1 "+3V3" V 5300 3050 50  0000 L CNN
F 2 "" H 5300 2900 50  0001 C CNN
F 3 "" H 5300 2900 50  0001 C CNN
	1    5300 2900
	0    1    1    0   
$EndComp
Text GLabel 4300 2700 0    50   Input ~ 0
ESP_CH_PD
Text GLabel 4300 2800 0    50   Input ~ 0
ESP_RST
Text GLabel 4300 3100 0    50   Input ~ 0
ESP_GIO0
Text GLabel 4300 3400 0    50   Input ~ 0
ESP_GPIO2
$EndSCHEMATC