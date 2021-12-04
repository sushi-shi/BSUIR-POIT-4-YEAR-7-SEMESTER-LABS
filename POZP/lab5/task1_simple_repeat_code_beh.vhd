----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/27/2021 04:13:26 PM
-- Design Name: 
-- Module Name: task1_simple_repeat_code_beh - Behavioral
-- Project Name: 
-- Target Devices: 
-- Tool Versions: 
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
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity REPEAT_ENCODER is
    generic(bits: integer := 4);
    Port (
        Qin: in STD_LOGIC_VECTOR(0 to bits - 1);
        Qout: out STD_LOGIC_VECTOR(0 to 2 * bits - 1)
    );
end REPEAT_ENCODER;

architecture Behavioral of REPEAT_ENCODER is
    signal q_t: STD_LOGIC_VECTOR(0 to 2 * bits - 1);
begin

    main: process(Qin)
    begin
        q_t <= Qin & Qin;
    end process;
	 
    Qout <= q_t;
end Behavioral;
