----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/28/2021 06:45:58 PM
-- Design Name: 
-- Module Name: tb_task3_ram_lifo - Behavioral
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

entity tb_task3_ram_lifo is
--  Port ( );
end tb_task3_ram_lifo;

architecture Behavioral of tb_task3_ram_lifo is
    constant period: time := 50ns;
    constant address: integer := 2;
    constant data: integer := 4;
    
    signal CLK: STD_LOGIC := '0';
    signal PUSH: STD_LOGIC := '0';
    signal POP: STD_LOGIC := '0';
    signal Din: STD_LOGIC_VECTOR(data - 1 downto 0);
    signal Dout: STD_LOGIC_VECTOR(data - 1 downto 0);

begin
    uut_1: entity work.task3_ram_lifo_beh 
		generic map (address, data) 
      port map (CLK, PUSH, POP, Din, Dout);
    
    CLK_proc: process
    begin
        CLK <= '0';
        wait for period / 2;
        CLK <= '1';
        wait for period / 2;
    end process;
    
    test_proc: process
    begin 
        PUSH <= '1';
		  
        Din <= "0000";  
        wait for period;
        Din <= "0001";  
        wait for period;
		  Din <= "0010";  
        wait for period;
		  Din <= "0011";  
        wait for period;
		  
        PUSH <= '0';
		  Din <= "ZZZZ";

        wait for period;
        
		  POP <= '1';
        wait for 5 * period;
        
        wait;
    end process;
end Behavioral;
