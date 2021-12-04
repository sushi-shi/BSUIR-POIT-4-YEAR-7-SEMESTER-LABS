----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/27/2021 03:19:38 PM
-- Design Name: 
-- Module Name: task1_even_check_code_beh - Behavioral
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

entity EVEN_ENCODER is
    generic(i: integer := 4);
    Port (
        Qin: in STD_LOGIC_VECTOR(0 to 2 ** i - 1);
		  
        Qout: out STD_LOGIC_VECTOR(0 to 2 ** i)
    );
end EVEN_ENCODER;

architecture Behavioral of EVEN_ENCODER is
    signal q_t: STD_LOGIC_VECTOR(0 to 2 ** i) := (others => '0');
begin
    main: process(Qin)
        variable tmp_xor: STD_LOGIC;
    begin
        tmp_xor := '0';
		  
        for i in 0 to 2 ** i - 1 loop
            tmp_xor := tmp_xor xor Qin(i);
        end loop;
		  
        q_t <= Qin & tmp_xor;
		  
    end process;
	 
    Qout <= q_t;
	 
end Behavioral;
