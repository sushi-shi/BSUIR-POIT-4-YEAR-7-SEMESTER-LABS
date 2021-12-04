----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/28/2021 06:45:26 PM
-- Design Name: 
-- Module Name: task3_ram_beh - Behavioral
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
use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity task3_ram_beh is
    generic (
        address: integer := 2;
        data: integer := 4
    );
    Port (
        CLK: in STD_LOGIC;
        RW: in STD_LOGIC;
        A: in STD_LOGIC_VECTOR(address - 1 downto 0);
		  Din: in STD_LOGIC_VECTOR(data - 1 downto 0);
        Dout: out STD_LOGIC_VECTOR(data - 1 downto 0)
    );
end task3_ram_beh;

architecture Behavioral of task3_ram_beh is
    type memory is array(0 to 2 ** address - 1) of STD_LOGIC_VECTOR(data - 1 downto 0);
    signal ram: memory;
    signal addr_index: integer range 0 to 2 ** address - 1;
begin

    addr_index <= TO_INTEGER(unsigned(A));

    read_proc: process(RW, ram, addr_index)
    begin
        if RW = '0' then
            Dout <= ram(addr_index);
        else
            Dout <= (others => 'Z');
        end if;
    end process;
    
    write_proc: process(CLK, RW, addr_index, Din)
    begin
        if RW = '1' then
            if rising_edge(CLK) then
                ram(addr_index) <= Din;
            end if;
        end if;
    end process;
end Behavioral;
