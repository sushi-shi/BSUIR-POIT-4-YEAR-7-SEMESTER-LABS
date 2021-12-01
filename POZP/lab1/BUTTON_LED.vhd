----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    22:34:46 09/11/2020 
-- Design Name: 
-- Module Name:    BUTTON_LED - Behavioral 
-- Project Name: 
-- Target Devices: 
-- Tool versions: 
-- Description: 
--
-- Dependencies: 
--
-- Revision: 
-- Revision 0.01 - File Created
-- Additional Comments: 
--
----------------------------------------------------------------------------------
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx primitives in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity BUTTON_LED is
    Port ( KEY1_in : in  STD_LOGIC;
           KEY2_in : in  STD_LOGIC;
           KEY3_in : in  STD_LOGIC;
           KEY4_in : in  STD_LOGIC;
           LED1_out : out  STD_LOGIC;
           LED2_out : out  STD_LOGIC;
           LED3_out : out  STD_LOGIC;
           LED4_out : out  STD_LOGIC);
end BUTTON_LED;
architecture Behavioral of BUTTON_LED is

begin
	LED1_out <= KEY1_in;
	LED2_out <= KEY2_in;
	LED3_out <= KEY3_in;
	LED4_out <= KEY4_in;
end Behavioral;