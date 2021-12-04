----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/27/2021 03:37:27 PM
-- Design Name: 
-- Module Name: tb_task1_even_check - Behavioral
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

entity tb_task1_even is
--  Port ( );
end tb_task1_even;

architecture Behavioral of tb_task1_even is
    constant i: integer := 2;
	 constant bits: integer := i ** 2;
    constant period: time := 100 ns;
    
    signal Q: STD_LOGIC_VECTOR(0 to i ** 2 - 1);
	 
    signal Q_code: STD_LOGIC_VECTOR(0 to i ** 2);
    signal Q_code_with_err: STD_LOGIC_VECTOR(0 to i ** 2);
	 
    signal Q_decode: STD_LOGIC_VECTOR(0 to i ** 2 - 1);
    signal ERR: STD_LOGIC;
    
begin
    uut_1: entity work.EVEN_ENCODER 
		generic map(i) 
		port map(Q, Q_code);
		
    uut_2: entity work.EVEN_DECODER
		generic map(i) 
		port map(Q_code_with_err, Q_decode, ERR);
		
    uut_3: entity work.MkError
		generic map(bits) 
		port map (Q_code, Q_code_with_err); 
		
    test_proc: process
    begin
        Q <= "1011";
        wait for period;
        Q <= "1111";
        wait for period;
        Q <= "0000";
        wait for period;
        Q <= "1110";
        wait for period;
    end process;

end Behavioral;
